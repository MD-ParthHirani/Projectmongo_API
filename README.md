How to use Employee API
-----------------------

Note: Replace "http://127.0.0.1:8080/" to  your address.

Note: For every request you have to provide USERNAME and PASSWORD in yor BasicAuth 



1. To get data of perticular employee use below api

   URL :
   ```
   http://127.0.0.1:8080/data/{email}
   ```

2. To get data of all employee use below api

   URL :
   ```
   http://127.0.0.1:8080/data
   63f498bdff3006eced448edf
   ```

3. To add new employee use below api
   
   URL :
   ```
   http://127.0.0.1:8080/insertdata
   ```

   Required body :
   ```
   {
    "Email": "Parth123@gmail.com",
    "Compass": "Parth@123",
    "Company_name": "minddeft",
    "Company_id": 456,
    "Company_address":kaliyabid" ,
    "Company_city":"bhavnagar",
   }
   }
   ```

4. To update the data of perticular employee use below api

   URL :
   ```
   http://127.0.0.1:8080/updatedata/email
   ```

   Required body :
   ```
   {
    "Company_name": "minddeft",
    "Company_id": 123,
    "Company_address":sola" ,
    "Company_city":"ahmedabad",
   }
   }
   ```


5. To delete perticular employee use below api

   URL :
   ```
   http://127.0.0.1:8080/deletedata/email
   ```
