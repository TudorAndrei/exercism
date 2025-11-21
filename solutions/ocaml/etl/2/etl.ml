let print (a, b) = 
  Printf.printf "(%s, \"%d\")\n"  a b

let print_line (letter, score) = 
  Printf.printf "(%s, \"%d\")\n"  letter score

let print_list sublist = 
  List.iter print_line sublist

let transform3 score letter = (Char.ASCII.lowercase letter, score);;

let transform_list  (score, letters) = 
  List.map (transform3 score) letters

let transform input_tuples = List.flatten (List.map transform_list input_tuples)

(* let () = *)
(*   let result = transform3 1 "TEXT"  in *)
(*   print result *)
(**)
(**)
(* let () = *)
(*   let result = transform_list (1, ["A"; "E"]) in *)
(*   List.iter print_line  result *)


(* let () = *)
(*   let result = transform [(1, ["A"; "E"]);(2, ["C"; "D"])] in *)
(*   List.iter print_line  result *)
