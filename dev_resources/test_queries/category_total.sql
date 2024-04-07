SELECT 
	category_id,
	display_name,
	sum(amount) as total
from transactions 
left join categories on transactions.category_id = categories.rowid
where not transactions.category_id = 0
group by category_id

