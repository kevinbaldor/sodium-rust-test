use sodium_rust::{SodiumCtx, StreamSink,CellLoop,Stream,Cell};

fn main() {
    let sodium_ctx    = SodiumCtx::new();

    let input : StreamSink<i64> = sodium_ctx.new_stream_sink();

    let (debug_listener, output) = sodium_ctx.transaction(|| {

        let output_cell_loop   : CellLoop<Stream<i64>>   = sodium_ctx.new_cell_loop();
        let output = Cell::switch_s(&output_cell_loop.cell());
        let constant: Cell<i64> = sodium_ctx.new_cell(100);

        let new_output = output.snapshot(&output_cell_loop.cell(),
            |prime: &i64, old_output: &Stream<i64>|
                {
                    let prime = *prime;
                    println!("Adding filter stage for {}",prime);
                    old_output.filter(move |x: &i64| (*x%prime)!=0)
                }
        );

        let test: Stream<i64> = output.snapshot(&constant, |a: &i64,b:&i64| *a+*b);
        let l= test.listen(|x: &i64| println!("snapshot of constant {}",*x));

        output_cell_loop.loop_(&new_output.hold(input.stream()));

        (l,output)
    });


    let l = output.listen(|x:&i64| println!("output {}",x));

    for x in 2..20 {
        println!("Sending {}",x);
        input.send(x);
        println!();
    }
    l.unlisten();
    debug_listener.unlisten();
}
