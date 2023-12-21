use day_20::part2;

pub fn main() {
    let input = "broadcaster -> rt, jr, rp, jl
%ln -> rr
%mg -> lt
%xs -> mg
%rr -> rl, ql
%dp -> hh, ql
%hh -> ql, vb
%cp -> ql, ln
%jr -> xs, ql
%vb -> ql
%vh -> ql, dp
%lt -> ql, cp
%rl -> ql, vh
&ql -> ln, jr, xs, mg, vm
&vm -> zg";
    println!("Result: {:?}", part2::process(input));
}
