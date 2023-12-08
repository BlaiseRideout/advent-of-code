  let node_ids = nodes.keys().collect_vec();
  let node_name_to_id: HashMap<&String, usize> = node_ids
    .iter()
    .cloned()
    .enumerate()
    .map(|(x, y)| (y, x))
    .collect();
  let node_edges_by_id: Vec<(usize, usize)> = node_ids
    .iter()
    .cloned()
    .filter_map(|id| nodes.get(id))
    .map(|(left, right)| {
      (
        *node_name_to_id.get(left).unwrap(),
        *node_name_to_id.get(right).unwrap(),
      )
    })
    .collect_vec();
  let starts = node_ids
    .iter()
    .enumerate()
    .filter(|(_, node)| node.ends_with('A'))
    .map(|(i, _)| i)
    .collect_vec();

  directions
    .chars()
    .cycle()
    .scan(starts, |cur_nodes, cur_direction| {
      cur_nodes.iter_mut().for_each(|cur_node| {
        //dbg!(&cur_node);
        let cur_edges = node_edges_by_id[*cur_node];
        *cur_node = match cur_direction {
          'L' => cur_edges.0,
          _ => cur_edges.1,
        };
      });
      if cur_nodes.iter().all(|node| node_ids[*node].ends_with('Z')) {
        None
      } else {
        Some(0)
      }
    })
    .count()
    + 1