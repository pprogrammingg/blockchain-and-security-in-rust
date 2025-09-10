pragma circom 2.0.0;

template NonEqual(){
  signal input in0;
  signal input in1;
  signal inverse;
  // essential check (in0 - in1) is non-zero
  // note: all operations in finite field are modulou some prime operations
  // e.g y = 1/3 mod 7  --> 3y mod 7 = 1 -> y = 5 as an example
  inverse <-- 1 / (in0 - in1);
  inverse * (in0 - in1) === 1;
}

// All elements are unique in the array
template Distinct(n){
  signal input in[n];
  component nonEqual[n][n];
  // loop to compare each and every possible pair
  for (var i=0; i < n; i++){
   for (var j=0; j < i; j++){
     nonEqual[i][j] = NonEqual();
     // if <-- is used instead this is more like a tube that goes there but with hole in it
     // so prover can input anything they want. <== it is more like sealed pipe, it guarantees verifier input ends up in the
     // component
     nonEqual[i][j].in0 <== in[i];
     nonEqual[i][j].in1 <== in[j];
   } 
  }
}

// Enforce that 0 <= in < 16
template Bits4(){
    signal input in;
    signal bits[4];
    var bitsum = 0;
    for (var i = 0; i < 4 ; i++) {
       bits[i] <-- (in >> i) & 1;
       bits[i] * (bits[i] - 1) === 0;
       bitsum = bitsum + 2 ** i * bits[i];
    }
    bitsum === in;
}

// Enforce that 1 <= in <= 9
template OneToNine() {
    signal input in;
    component lowerBound = Bits4();
    component upperBound = Bits4();
    lowerBound.in <== in - 1;
    upperBound.in <== in + 6;
}

template Sudoku(n) {
   // solution is a 2D array
   signal input solution[n][n];
   // puzzle is same as solution with 0 as blank
   signal input puzzle[n][n];

   // ensure each solution cell number is in the range
   // how the solution is verified.
   component inRange[n][n];
   for (var i = 0; i < n; i++) {
     for (var j = 0; j < n; j++) {
        inRange[i][j] = OneToNine();
        inRange[i][j].in <== solution[i][j];
     }
   }

   // ensure puzzle and solution agree
    for (var i = 0; i < n; i++) {
       for (var j = 0; j < n; j++) {
           // check puzzle_cell * (puzzle_cell - solution_cell) === 0
           // basically means either puzzle_cell has to be 0
           // or puzzle_cell and solution_cell need to be equal
           puzzle[i][j] * (puzzle[i][j] - solution[i][j]) === 0;
       }
    }

    // ensure uniqueness in rows
    component distinct[n];
    for (var i = 0; i < n; i++) {
       // each row requires a new component
       distinct[i] = Distinct(n);
       for (var j = 0; j < n; j++) {
           // distinct component i at input j be assigned and constrained to solution's i,j
           distinct[i].in[j] <== solution[i][j];
       }
    }
}

component main {public[puzzle]} = Sudoku(9);


