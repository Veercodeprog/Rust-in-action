fn main() {
    let ctx_times = 2;
    let needle = "oo";
    let haystack = "\
        Every face, every shop,
6 bedroom window, public-house, and
7 dark square is a picture
8 feverishly turned--in search of what?
9 It is the same with books.
10 What do we seek
11 through millions of pages?";
    let mut tags: Vec<usize> = vec![]; // create a new vector
    let mut ctx: Vec<Vec<(usize, String)>> = vec![]; // create a new vector of vectors
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_times + 1);
            ctx.push(v);
        }
    }
    if (tags.is_empty()) {
        println!("No matches found");
        return;
    }
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_times);
            let upper_bound = tag + ctx_times;
            if (i >= lower_bound && i <= upper_bound) {
                let local_ctx = (i, line.to_string());
                ctx[j].push(local_ctx);
            }
        }
    }
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
