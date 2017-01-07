open Str

let [n; e] = read_line() |> Str.(split (regexp " ")) |> List.map int_of_string

let edges = Array.init n (fun _ -> Array.make n ~-1)

let _ =
  for _ = 1 to e do
    let [n1; n2; w] = read_line() |> Str.(split (regexp " ")) |> List.map int_of_string in
    edges.(n1-1).(n2-1) <- w
  done;

  for k = 0 to n-1 do
    for i = 0 to n-1 do
      if edges.(i).(k) <> ~-1 then
        for j = 0 to n-1 do
          if edges.(k).(j) <> ~-1 && (edges.(i).(j) = ~-1 || edges.(i).(k) + edges.(k).(j) < edges.(i).(j)) then
            edges.(i).(j) <- edges.(i).(k) + edges.(k).(j)
        done
    done
  done;

  let tests = read_line() |> int_of_string in

  for _ = 1 to tests do
    let [nt1; nt2] = read_line() |> Str.(split (regexp " ")) |> List.map int_of_string in
    edges.(nt1-1).(nt2-1) |> print_int;
    print_newline()
  done
