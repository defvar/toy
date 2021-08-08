#[test]
fn regex_example() {
    use regex::Regex;
    let line_regex = Regex::new(
        r"(?x)
    (?P<datetime>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d*[+-]\d{2}:\d{2})\s+
    (?P<level>\S+)\s+
    (?P<thread_name>\S+)\s+
    (?P<thread_id>\S+)\s+
    Task\{\s*task=(?P<task_id>\S+)\s*(graph=(?P<graph>\S+))?\s*(uri=(?P<uri>\S+))?\s*}:\s+
    (?P<target>\S+):\s+
    (?P<time>close\s*time\.busy=(?P<busy>\S+)\s*time\.idle=(?P<idle>\S+)?)?\s?
    (?P<message>.*)?
    ",
    )
    .unwrap();

    println!("{}", line_regex.to_string());

    let text1 = "2021-08-07T11:54:04.565605+00:00  INFO toy-worker ThreadId(06) Task{task=123 graph=example-tick uri=awaiter}: toy_executor::executor: all upstream finish. awaiter.";
    //let text2 = "2021-08-07T11:54:04.565668+00:00  INFO toy-worker ThreadId(06) Task{task=123 graph=example-tick uri=awaiter}: toy_core::task: close time.busy=0.00ns time.idle=6.04s";
    //let text3 = "2021-08-07T11:54:04.565880+00:00  INFO toy-worker ThreadId(06) Task{task=123 graph=example-tick}: toy_core::task: close time.busy=0.00ns time.idle=6.04s";

    let m = line_regex.captures(text1).unwrap();
    println!("datetime:{:?}", m.name("datetime").map(|x| x.as_str()));
    println!("level:{:?}", m.name("level").map(|x| x.as_str()));
    println!(
        "thread_name:{:?}",
        m.name("thread_name").map(|x| x.as_str())
    );
    println!("thread_id:{:?}", m.name("thread_id").map(|x| x.as_str()));
    println!("task_id:{:?}", m.name("task_id").map(|x| x.as_str()));
    println!("graph:{:?}", m.name("graph").map(|x| x.as_str()));
    println!("uri:{:?}", m.name("uri").map(|x| x.as_str()));
    println!("target:{:?}", m.name("target").map(|x| x.as_str()));
    println!("busy:{:?}", m.name("busy").map(|x| x.as_str()));
    println!("idle:{:?}", m.name("idle").map(|x| x.as_str()));
    println!("message:{:?}", m.name("message").map(|x| x.as_str()));
}
