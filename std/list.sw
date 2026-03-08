is_tuple := |x| (x + 0) == x ? 0 : 1;
export is_tuple;

pop := |tuple| {
    i := #tuple - 1;
    n := tuple.i;
    tuple := tuple - i;
    (tuple, n)
};
export pop;


push_front := |tuple, new_front| {
    new_tuple := new_solo(new_front);
    result := loop(i := 0, new_tuple := new_tuple) {
        #tuple <= i ? new_tuple : recur(i + 1, new_tuple + tuple.i)
    };
    result
};
export push_front;


new_solo := |element| pop((element, 0)).0;
export new_solo;

new_empty := || new_solo(0) - 0;
export new_empty;


flatten := |tuple| {
    loop(i := 0, acc := (new_solo(0) - 0)) {
        i >= #tuple
            ? acc 
            : {
                item := tuple.i;
                is_tuple(item)
                    ? recur(i + 1, acc + flatten(item))
                    : recur(i + 1, acc + item)
            }
    }
};
export flatten;

sort := |tuple| {
    tuple := flatten(tuple);
    #tuple < 2 ? tuple : {
        pivot := #tuple / 2;
        left := new_empty();
        right := new_empty();
        tuples := loop(i := 0, left := left, right := right) {
            i >= #tuple ? (left, right) : {
                check_l := tuple.i < tuple.pivot;
                check_r := i != pivot ? tuple.i >= tuple.pivot : tuple.i > tuple.pivot;
                recur(i + 1, check_l ? left + tuple.i : left, check_r ? right + tuple.i : right)
            }
        };
        tuples.0 := sort(tuples.0);
        tuples.1 := sort(tuples.1);
        tuples.0 + tuple.pivot + tuples.1

    }
};
export sort;
