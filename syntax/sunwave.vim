" Language: sunwave (sw)

if exists("b:current_syntax")
  finish
endif

" Keywords
syntax keyword swKeyword loop recur module export import
syntax keyword swConditional ? :

" Numbers (Integers and Floats)
syntax match swNumber "\v<\d+>"
syntax match swNumber "\v<\d+\.\d+>"

" Operators
syntax match swOperator "\v\:\="
syntax match swOperator "\v[\+\-\*\/\%\=\>\<]"
syntax match swOperator "\v\=\="
syntax match swOperator "\v\>\="
syntax match swOperator "\v\<\="

" Delimiters and Lambdas
syntax match swDelimiter "\v[|(){},.]"

" Comments (Assuming // style)
syntax match swComment "\v//.*$"

" Highlighting Links
highlight default link swKeyword Statement
highlight default link swConditional Conditional
highlight default link swNumber Constant
highlight default link swOperator Operator
highlight default link swDelimiter Delimiter
highlight default link swComment Comment

let b:current_syntax = "sw"
