use std::time::Duration;

fn main() {
    trpl::run(async {
        let f1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                //trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let f2 = async {
            for i in 1..10 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(f1, f2).await;
    });
}
