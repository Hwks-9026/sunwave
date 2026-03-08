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

