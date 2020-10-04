use runner::common::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::string::ToString;
extern crate strum;
use strum_macros::{Display, EnumIter};

fn main() {
    // TODO find a more challenging set of tasks, and try to find some egde cases.
    // let mut t1 = Task {
    //     id: "T1".to_string(),
    //     prio: 1,
    //     deadline: 100,
    //     inter_arrival: 100,
    //     trace: Trace {
    //         id: "T1".to_string(),
    //         start: 0,
    //         end: 10,
    //         inner: vec![],
    //     },
    // };

    // let t2 = Task {
    //     id: "T2".to_string(),
    //     prio: 2,
    //     deadline: 200,
    //     inter_arrival: 200,
    //     trace: Trace {
    //         id: "T2".to_string(),
    //         start: 0,
    //         end: 30,
    //         inner: vec![
    //             Trace {
    //                 id: "R1".to_string(),
    //                 start: 10,
    //                 end: 20,
    //                 inner: vec![Trace {
    //                     id: "R2".to_string(),
    //                     start: 12,
    //                     end: 16,
    //                     inner: vec![],
    //                 }],
    //             },
    //             Trace {
    //                 id: "R1".to_string(),
    //                 start: 22,
    //                 end: 28,
    //                 inner: vec![],
    //             },
    //         ],
    //     },
    // };

    // let t3 = Task {
    //     id: "T3".to_string(),
    //     prio: 3,
    //     deadline: 50,
    //     inter_arrival: 50,
    //     trace: Trace {
    //         id: "T3".to_string(),
    //         start: 0,
    //         end: 30,
    //         inner: vec![Trace {
    //             id: "R2".to_string(),
    //             start: 10,
    //             end: 20,
    //             inner: vec![],
    //         }],
    //     },
    // };

    let t2 = Task {
        id: "T2".to_string(),
        prio: 2,
        deadline: 200,
        inter_arrival: 200,
        trace: Trace {
            id: "T2".to_string(),
            start: 0,
            end: 30,
            inner: vec![],
        },
    };

    let t3 = Task {
        id: "T3".to_string(),
        prio: 3,
        deadline: 10,
        inter_arrival: 10,
        trace: Trace {
            id: "T3".to_string(),
            start: 0,
            end: 3,
            inner: vec![],
        },
    };

    let t4 = Task {
        id: "T4".to_string(),
        prio: 4,
        deadline: 15,
        inter_arrival: 15,
        trace: Trace {
            id: "T4".to_string(),
            start: 0,
            end: 2,
            inner: vec![],
        },
    };

    let tasks: Tasks = vec![t2, t3, t4];
    let (mut ip, mut tr) = pre_analysis(&tasks);
    if check_scheduability(&tr, &ip, &tasks) {
        println!("The system is scheduable");
    } else {
        println!("The system is NOT scheduable");
    }

    //println!("{:?}", gatherInfoFromTasks(&tr, &ip, &tasks));

    let vector_of_detailed_tasks = gatherInfoFromTasks(&tr, &ip, &tasks);
    for mut task in vector_of_detailed_tasks {
        println!("{:?}", task.get(0).unwrap());
        println!("{:?}", task.get(1).unwrap());
        println!("{:?}", task.get(2).unwrap());
        println!("{:?}", task.get(3).unwrap());
        println!("{:?}", task.get(4).unwrap());
    }
}

//Returns the worst execution time, C(t)
fn worst_case_execution_time(task: &Task) -> u32 {
    let start_time = task.trace.start;
    let end_time = task.trace.end;
    return end_time - start_time;
}

//Returns the cpu load L(t) = C(t) / A(t) per task
fn cpu_load(task: &Task) -> f32 {
    let wcet = worst_case_execution_time(task) as f32;
    let inter_arrival_time = task.inter_arrival as f32;
    return wcet / inter_arrival_time;
}

//Returns the total cpu load, Ltot
fn total_cpu_load(list_of_tasks: &Tasks) -> f32 {
    let mut sum = 0.0;
    for i in list_of_tasks {
        sum = sum + cpu_load(&i);
    }
    return sum;
}

fn blocking_time(task: &Task, tr: &TaskResources, ip: &IdPrio, list_of_tasks: &Tasks) -> u32 {
    let mut block_time = 0;
    let mut to_find = vec![];
    let mut task_prio = 0;
    let resource_prio = 0;
    let mut worst_case_execution_time_for_resource: Option<&u32> = Some(&0);

    match ip.get(&task.id) {
        Some(prio) => {
            task_prio = *prio;
        }
        _ => (),
    }

    match tr.get(&task.id) {
        Some(resource) => {
            for r in resource {
                to_find.insert(0, r);
            }
        }
        _ => (),
    }
    for (j, item) in to_find.iter().enumerate() {
        match ip.get(to_find[j]) {
            Some(resource) => {
                let resource_prio = resource;
                if (resource_prio >= &task_prio) {
                    for key in tr.keys() {
                        if key == &task.id {
                            break;
                        }
                        match ip.get(key) {
                            Some(prio) => {
                                let task_compare_prio = prio;
                                let mut time_for_resource = vec![];
                                if task_compare_prio < &task_prio {
                                    for task in list_of_tasks {
                                        if &task.id == key {
                                            time_for_resource = find_resource(
                                                task_prio,
                                                &task_compare_prio,
                                                &task.trace,
                                                key,
                                                to_find[j],
                                                &mut time_for_resource,
                                            );
                                        }
                                    }
                                }
                                worst_case_execution_time_for_resource =
                                    time_for_resource.iter().max();
                                match worst_case_execution_time_for_resource {
                                    Some(max) => block_time = block_time + *max,
                                    None => (),
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }
    return block_time;
}

fn preemption(
    task: &Task,
    tr: &TaskResources,
    ip: &IdPrio,
    list_of_tasks: &Tasks,
    preemption_vec: &mut std::vec::Vec<f32>,
    aprox: bool,
) -> f32 {
    let mut task_prio = 0;
    let mut compare_task_prio = 0;

    match ip.get(&task.id) {
        Some(prio) => {
            task_prio = *prio;
        }
        None => (),
    }

    for compare_task in list_of_tasks {
        match ip.get(&compare_task.id) {
            Some(prio) => {
                compare_task_prio = *prio;
            }
            None => (),
        }

        if compare_task_prio > task_prio {
            if !aprox {
                let block_time = blocking_time(&task, &tr, &ip, &list_of_tasks);
                let wcet = worst_case_execution_time(&task);
                let mut preemption_result = preemption_rec(
                    &task,
                    &compare_task,
                    block_time as f32,
                    wcet as f32,
                    (block_time as f32 + wcet as f32),
                    0.0,
                );
                &preemption_vec.push(preemption_result);
            } else if aprox {
                let deadline_interarrival_qouta =
                    *&compare_task.deadline as f32 / *&compare_task.inter_arrival as f32;
                &preemption_vec.push(
                    worst_case_execution_time(&compare_task) as f32
                        * deadline_interarrival_qouta.ceil(),
                );
            }
        }
    }
    let mut sum: f32 = preemption_vec.iter().sum();
    preemption_vec.clear();

    return sum;
}

fn response_time(preemption: f32, worst_case_execution_time: u32, blocking_time: u32) -> f32 {
    return preemption + worst_case_execution_time as f32 + blocking_time as f32;
}

fn preemption_rec(
    task: &Task,
    compare_task: &Task,
    blocking_time: f32,
    wcet: f32,
    mut current_respone_time: f32,
    mut previous_respone_time: f32,
) -> f32 {
    if (current_respone_time - blocking_time - wcet) > *&task.deadline as f32 {
        panic!();
    } else if current_respone_time == previous_respone_time
        && current_respone_time == blocking_time + wcet
    {
        return current_respone_time;
    } else if current_respone_time == previous_respone_time
        && current_respone_time > blocking_time + wcet
    {
        return current_respone_time - blocking_time - wcet;
    }
    previous_respone_time = current_respone_time;
    current_respone_time = blocking_time
        + wcet
        + ((current_respone_time / *&compare_task.inter_arrival as f32)
            * worst_case_execution_time(&compare_task) as f32)
            .ceil();
    return preemption_rec(
        task,
        compare_task,
        blocking_time,
        wcet,
        current_respone_time,
        previous_respone_time,
    );
}

fn find_resource(
    task_prio: u8,
    task_compare_prio: &u8,
    trace: &Trace,
    key: &str,
    to_find: &str,
    time_for_resource: &mut Vec<u32>,
) -> Vec<u32> {
    for i in &trace.inner {
        if &i.id == to_find {
            time_for_resource.push(&i.end - &i.start);
        }
        find_resource(
            task_prio,
            &task_compare_prio,
            i,
            key,
            to_find,
            time_for_resource,
        );
    }
    return time_for_resource.to_vec();
}

// Checks if the "system" is scheduable based on:
// Ltot < 1, otherwise more than 100% of cpu is used
// R(t) < D(t), for all tasks. (R(t) > D(t) implies a deadline miss.)

fn check_scheduability(tr: &TaskResources, ip: &IdPrio, list_of_tasks: &Tasks) -> bool {
    let mut preemption_vector = vec![];
    for task in list_of_tasks {
        let r = response_time(
            preemption(
                &task,
                &tr,
                &ip,
                &list_of_tasks,
                &mut preemption_vector,
                false,
            ),
            worst_case_execution_time(&task),
            blocking_time(&task, &tr, &ip, &list_of_tasks),
        );
        if r < task.deadline as f32 {
            return false;
        }
    }
    if total_cpu_load(list_of_tasks) < 1.0 {
        return true;
    }
    return false;
}

#[derive(Debug, Display, Clone)]
enum DetailedTask {
    Task(Task),
    R(f32),
    C(u32),
    B(u32),
    I(f32),
}
fn gatherInfoFromTasks(
    tr: &TaskResources,
    ip: &IdPrio,
    list_of_tasks: &Tasks,
) -> Vec<Vec<DetailedTask>> {
    let mut task_vec = vec![];
    for task in list_of_tasks.clone() {
        let mut vec = vec![];
        let task_copy = task.clone();
        let mut preemption_vec = vec![];
        let aprox = false;
        vec = vec![
            DetailedTask::Task(task_copy),
            DetailedTask::R(response_time(
                preemption(&task, &tr, &ip, &list_of_tasks, &mut preemption_vec, aprox),
                worst_case_execution_time(&task),
                blocking_time(&task, &tr, &ip, &list_of_tasks),
            )),
            DetailedTask::C(worst_case_execution_time(&task)),
            DetailedTask::B(blocking_time(&task, &tr, &ip, &list_of_tasks)),
            DetailedTask::I(preemption(
                &task,
                &tr,
                &ip,
                &list_of_tasks,
                &mut preemption_vec,
                false,
            )),
        ];
        task_vec.push(vec);
    }
    return task_vec;
}
