// Substantial parts of this code are adapted from the `logos` crate's benchmarks.
// Logos is licensed under the MIT License and Apache License 2.0.
//
// The original source can be found at:
// https://github.com/maciejhirsz/logos/blob/master/tests/benches/bench.rs
// Copyright (c) 2018 Maciej Hirsz <maciej.hirsz@gmail.com>
//
// `scnr2` is also licensed under the MIT License and Apache License 2.0.

use codspeed_criterion_compat::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use scnr2::scanner;

scanner! {
    BenchScanner {
        mode INITIAL {
            token r"[ \n\t\f]" => 1; // InvalidToken
            token "[a-zA-Z_$][a-zA-Z0-9_$]*" => 2; // Identifier
            token r#""([^"\\]|\\t|\\u|\\n|\\")*""# => 3; // String
            token "private" => 4; // Private
            token "primitive" => 5; // Primitive
            token "protected" => 6; // Protected
            token "in" => 7; // In
            token "instanceof" => 8; // Instanceof
            token r"\." => 9; // Accessor
            token r"\.\.\." => 10; // Ellipsis
            token r"\(" => 11; // ParenOpen
            token r"\)" => 12; // ParenClose
            token r"\{" => 13; // BraceOpen
            token r"\}" => 14; // BraceClose
            token r"\+" => 15; // OpAddition
            token r"\+\+" => 16; // OpIncrement
            token "=" => 17; // OpAssign
            token "==" => 18; // OpEquality
            token "===" => 19; // OpStrictEquality
            token "=>" => 20; // FatArrow
        }
    }
}

static SOURCE: &str = "
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
foobar(protected primitive private instanceof in) { + ++ = == === => }
";

static IDENTIFIERS: &str = "It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton \
                            It was the year when they finally immanentized the Eschaton";

static STRINGS: &str = r#""tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree." "tree" "to" "a" "graph" "that can" "more adequately represent" "loops and arbitrary state jumps" "with\"\"\"out" "the\n\n\n\n\n" "expl\"\"\"osive" "nature\"""of trying to build up all possible permutations in a tree.""#;

static CANDIDATES: [(&str, &str); 3] = [
    ("identifiers", IDENTIFIERS),
    ("keywords_operators_and_punctuators", SOURCE),
    ("strings", STRINGS),
];

#[allow(unused_must_use)]
fn iterate(s: &str) {
    use bench_scanner::BenchScanner;

    let scanner = BenchScanner::new();
    let find_iter = scanner.find_matches(s, 0, &BenchScanner::match_function);

    for token in find_iter {
        black_box(token);
    }
}

fn count_ok(s: &str) -> usize {
    use bench_scanner::BenchScanner;

    let scanner = BenchScanner::new();
    let find_iter = scanner.find_matches(s, 0, &BenchScanner::match_function);
    find_iter
        .filter_map(|ma| if ma.token_type > 1 { Some(ma) } else { None })
        .count()
}

fn bench_iterate(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterate");

    for (name, source) in CANDIDATES {
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_with_input(name, &source, |b, &s| b.iter(|| iterate(s)));
    }
}

fn bench_count_ok(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_ok");

    for (name, source) in CANDIDATES {
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_with_input(name, &source, |b, &s| b.iter(|| count_ok(s)));
    }
}

criterion_group!(benches, bench_iterate, bench_count_ok);
criterion_main!(benches);
