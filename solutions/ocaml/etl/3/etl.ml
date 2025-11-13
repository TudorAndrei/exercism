let print (a, b) = 
  Printf.printf "(%s, \"%d\")\n"  a b

let print_line (letter, score) = 
  Printf.printf "(%s, \"%d\")\n"  letter score

let print_list sublist = 
  List.iter print_line sublist

let transform3 score letter = (Char.lowercase_ascii letter, score);;

let transform_list  (score, letters) = 
  List.map (transform3 score) letters

let transform input_tuples = List.flatten (List.map transform_list input_tuples)
