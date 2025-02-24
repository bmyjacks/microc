%%
begin "BEGIN"
end "END"
read "READ"
write "WRITE"
\( "LPAREN"
\) "RPAREN"
\; "SEMICOLON"
\, "COMMA"
:= "ASSIGNOP"
\+ "PLUSOP"
\- "MINUSOP"
\* "MULTIOP"
\/ "DIVIDEOP"
[a-zA-Z][a-zA-Z0-9]{0,31} "ID"
-?[0-9]+ "INTLITERAL"
[ \t\r\n]+ ;
--[^\n]*[\n\r]? ;
