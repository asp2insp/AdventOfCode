
int main() {
    int target, one, two, three, x, ip = 0;
    // seti 0 3 4
    x = 3;
    // bori 4 65536 1
    one = x | 65536;
    // seti 2024736 3 4
    x = 2024736;
    // bani 1 255 2
    two = one & 255;
    // addr 4 2 4
    x = x + 2;
    // bani 4 16777215 4
    x = x & 16777215;
    // muli 4 65899 4
    x = x * 65899;
    // bani 4 16777215 4
    x = x & 16777215;
    // gtir 256 1 2
    two = 255 > one ? 1 : 0;
    // addr 2 5 5
    ip = ip + two;
    // addi 5 1 5
    ip = ip + 1;
    // seti 27 7 5
    ip = 27;
    // seti 0 1 2
    two = 0;
    // addi 2 1 3
    three = two + 1;
    // muli 3 256 3
    three = three * 256;
    // gtrr 3 1 3
    three = three > one ? 1 : 0;
    // addr 3 5 5
    ip = ip + three;
    // addi 5 1 5
    ip = ip + 1;
    // seti 25 2 5
    ip = 25;
    // addi 2 1 2
    two = two + 1;
    // seti 17 0 5
    ip = 17;
    // setr 2 3 1
    one = two;
    // seti 7 9 5
    ip = 7;
    // eqrr 4 0 2
    two = x == target ? 1 : 0;
    // addr 2 5 5
    ip = ip + 2;
    // seti 5 6 5
    ip = 5;
}