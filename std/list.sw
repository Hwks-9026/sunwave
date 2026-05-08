is_tuple := |x| (x + 0) == x ? false : true;
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


new_solo := |element| (element, 0) - 1;
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

fold_left := |f, initial, tuple| {
    loop(i := 0, acc := initial) {
        i >= #tuple 
            ? acc 
            : recur(i + 1, f(acc, tuple.i))
    }
};
export fold_left;

reverse := |tuple| {
    new := new_empty(); 
    loop(i := 0, new := new) {
        i >= #tuple ? new : {
            j :=  #tuple - 1 - i;
            recur(i + 1, new + tuple.j)
        }
    }
};
export reverse;

map := |f, tuple| {
    fold_left(|acc, x| acc + f(x), new_empty(), tuple)
};
export map;

import "std/math"

windows := |tuple, n| {
    new := new_empty();

    loop(i := 0, new := new) {
        i >= #tuple ? new : recur(i + 1, {

            new + new_solo(loop(j := 0, current := new_empty()) {
                j >= n ? current : {

                    target := std.math.core.modulo(i + j, #tuple);
                    recur(j + 1, current + tuple.target)
                
                }
            })

        })
    }

};
export windows;

pairs := |tuple| {windows(tuple, 2)};
export pairs;

filter := |tuple, predicate_lambda| {
    new := new_empty();
    
    loop(i := 0, new := new) {
        i >= #tuple ? new : recur(i + 1, predicate_lambda(tuple.i) ? new + tuple.i : new)
    }
};
export filter;


slice := |tuple, start, end| {
    new := new_empty();
    (end >= #tuple) | (start >= #tuple) | (end < start) | (start < 0) | (end < 0) ? new : {
        loop(i := start, new := new_empty()) {
            i > end ? new : recur(i + 1, new + tuple.i)
        }
    }
};
export slice;

find := |tuple, predicate_lambda| {
    loop(i := 0) {
        i >= #tuple 
            ? none
            : predicate_lambda(tuple.i) 
                ? tuple.i 
                : recur(i + 1)
    }
};
export find;

