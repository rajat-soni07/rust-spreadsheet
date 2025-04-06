use std::collections::VecDeque;
pub fn topo_sort(adj: &Vec<Vec<i32>>,cell:i32, indegree: &mut Vec<i32>) -> Vec<i32>{
    //arguments- adj(vector of vector)-- adjacency list , indegree(vector) -- zero initialized vector 
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(cell); let mut is_cycle=0; let mut ct:i32=1;

    while q.len()>0{
        if is_cycle==1{break;}
        let node = q.pop_front().unwrap();ct+=1;
        for c in &adj[node as usize]{
            if*c==cell{is_cycle=1;break;}
            if indegree[*c as usize]==0{
                q.push_back(*c);
            }
            indegree[*c as usize]+=1;
        }
    }

    let mut res: Vec<i32> = vec![0; ct as usize];q.push_back(cell);
    if is_cycle==1{
        res[0]=-1;
        // we need to revert back changes in indegree
        while q.len()>0{
            let node=q.pop_front().unwrap();
            for c in &adj[node as usize]{
                if*c == cell{break;}
                if indegree[*c as usize]>0{
                    q.push_back(*c);
                }
                indegree[*c as usize]=0;

            }
        }
        return res;
    }
    res[0]=ct-1; let mut leng=1;

    while q.len()>0{
        let node = q.pop_front().unwrap();
        // print!("{} ",node);
        res[leng as usize]=node;leng+=1;
        for c in &adj[node as usize]{
            indegree[*c as  usize]-=1;
            if indegree[*c as usize]==0{
                q.push_back(*c);
            }
            
        }

    }
    return res;
}


