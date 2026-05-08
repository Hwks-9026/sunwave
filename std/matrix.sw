import "std/list"
zeros := |cols, rows| {
    m := rows;
    n := cols;
    matrix := loop(i := 0, matrix := std.list.new_empty())  {
        i >= m ? matrix : {
            row := loop(j := 0, row := std.list.new_empty())  {
                j >= n ? row : {
                    recur(j + 1, row + 0) 
                }
            };
            recur(i + 1, matrix + std.list.new_solo(row))
        } 
    };
    matrix
};
export zeros;

zeros_square := |n| {zeros(n, n)};
export zeros_square;


rotate_180 := |matrix| {
    std.list.is_tuple(matrix.0) == 0 ? matrix : {
        loop(i := (#matrix - 1), new := std.list.new_empty()) {
            i < 0 ? new : {
                recur(i - 1, new + std.list.new_solo(std.list.reverse(matrix.i)))
            }
        }
    }
};
export rotate_180;

zeros_n_d := |dimensions_tuple| {
    #dimensions_tuple == 2 ? zeros(dimensions_tuple.0, dimensions_tuple.1) : {
    
        cur_dim_idx := #dimensions_tuple - 1;
        cur_dim := dimensions_tuple.cur_dim_idx;
        sub_dim := std.list.new_solo(zeros_n_d(dimensions_tuple - cur_dim_idx));
        cur := loop(i := 0, row := std.list.new_empty())  {
            i >= cur_dim ? row : {
                recur(i + 1, row + sub_dim) 
            }
        };
        cur
    }
};
export zeros_n_d;


tridiag := |diag, sub_diag, sup_diag, n| {
    A := loop(i := 0, A := zeros(n, n)) {
        i >= n ? A : {
            A.i.i := diag;
            recur(i+1, A);
        }
    };
    A
};

export tridiag;

