use criterion::{criterion_group, criterion_main, Criterion};
use rust_chess_engine::{agent::MinmaxAgent, game::Game};

pub fn run_two_rounds()
{
    let agent_white = MinmaxAgent::new(5);
    let agent_black = MinmaxAgent::new(5);

    let mut game = Game::new(agent_white, agent_black);
    game.next_round();
}

pub fn benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("MinMaxAgent benchmark");
    group.sample_size(10);
    group.bench_function("run_two_rounds with AI agents", |b| b.iter(|| run_two_rounds()));
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

