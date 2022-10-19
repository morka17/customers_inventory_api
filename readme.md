#   Inventory API


> ### Enpoints 
- /customers 
    -  GET -> list all customers in data store 
    -  POST -> create new customer and insert into data store 
- /customers/{guid} 
    - GET -> list info for a customer 
    - POST -> update information for a customer 
    - DELETE -> remove customer from data store
  

> ### Handlers
>  - list_customers -> return a list all customers in database create_customer -> create a new customer and add it to the database 
> - get_customer -> return the details of a single 
> - customer update_customer -> update the details of a single customer 
> - delete_customer -> delete a customer from the database


> ### Database 
__Using a in-memory data store to share data across the route handlers__
```json
{
    "guid": "String",
    "first_name": "String",
    "last_name": "String",
    "email": "String",
    "address": "String"
}
 ```