fn main()
{
    let mut matrix_val=[[1,2,3], 
                    [4,5,6], 
                    [7,8,9]];

    //matrix_array(&matrix_val); 

    // mutable iterator is iter_mut()

    for row in matrix_val.iter_mut()
    {
        for num in row.iter_mut()
        {
            *num += 10;  // reference and dereference 

            print!("{}\t", num)
        }

        println!();

    }

}

fn matrix_array(mat: &[[i32; 3]; 3])
{
    for rows in 0..mat.len()
    {
        for columns in 0..mat[rows].len()
        {
            println!("The row {} with column {} have value {} ", rows, columns, mat[rows][columns]);
        }
    }
}