SELECT 
	payee, 
	amount,
	date,
	notes,
	account_id,
	ifnull(categories.display_name, '') as category_display_name
from transactions 
left join categories on transactions.category_id = categories.ROWID 