pub fn should_task_run(this_tag: String, tags: &Option<Vec<String>>) -> bool {
    if let Some(t) = &tags {
        return t.contains(&this_tag) || t.is_empty();
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_if_task_should_run() {
        /// true if in tag vec
        let tags = Some(
            vec!["foo", "bar", "test-tag"]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
        );
        let is_in_vec = should_task_run("test-tag".to_string(), &tags);
        assert!(is_in_vec);

        /// true if vec is empty
        let vec_empty = should_task_run("test-tag".to_string(), &None);
        assert!(vec_empty);

        /// false is vec not empty and tag is missing
        let vec_not_empty_not_in = should_task_run("bang".to_string(), &tags);
        dbg!(&vec_not_empty_not_in);
        assert!(!vec_not_empty_not_in);
    }
}
