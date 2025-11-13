let print (a, b) = 
  Printf.printf "(%c, \"%d\")\n"  a b

let print_line (letter, score) = 
  Printf.printf "(%c, \"%d\")\n"  letter score

let print_list sublist = 
  List.iter print_line sublist

let transform3 score letter = (Char.lowercase_ascii letter, score);;

let transform_list  (score, letters) = 
  List.map (transform3 score) letters

let compare a b = 
  if fst a = fst b 
    then 0
  else 
    (if fst a > fst b
    then 1
  else 0)

let transform input_tuples = 
  List.sort compare  (List.flatten (List.map transform_list input_tuples)) 

(* let () = *)
(*   let result = transform3 1 "TEXT"  in *)
(*   print result *)
(**)
(**)
(* let () = *)
(*   let result = transform_list (1, ["A"; "E"]) in *)
(*   List.iter print_line  result *)


(* let () = *)
(*   let result = transform [(1, ['A'; 'E']); (2, ['D'; 'G'])] in  *)
(*   List.iter print_line  result *)
