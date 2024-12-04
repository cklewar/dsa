/*
O(V + E)
*/

use std::collections::HashSet;

fn main() {
    fn course_schedule(num_courses: usize, prerequisites: Vec<Vec<usize>>) -> bool {
        let mut graph: Vec<Vec<usize>> = vec![vec!(); num_courses as usize];

        for p in prerequisites.iter() {
            let ai: usize = p[0];
            let bi: usize = p[1];
            graph[ai].push(bi);
        }
        println!("graph: {:?}", graph);

        fn dfs(
            course: usize,
            visited: &mut HashSet<usize>,
            path: &mut HashSet<usize>,
            graph: &mut Vec<Vec<usize>>,
        ) -> bool {
            if path.contains(&course) {
                return false;
            }
            if visited.contains(&course) {
                return true;
            }

            path.insert(course);
            let mut x: bool = true;

            for p in graph[course].clone() {
                x = dfs(p, visited, path, graph)
            }
            
            x
        }

        let mut x: bool = true;

        for course in 0..num_courses as usize {
            x = dfs(course, &mut HashSet::new(), &mut HashSet::new(), &mut graph);
            if x == false {
                break;
            }
        }

        x
    }

    //let result = course_schedule(2, vec![vec![1, 0]]);
    let result = course_schedule(2, vec![vec![1, 0], vec![0, 1]]);
    println!("Are courses schedulable? {:?}", result);
}
