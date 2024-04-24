# aidich.pro

Client để chạy dịch nhiều file txt. Có 2 cách.

1. Dùng file AIdichdotProClientRust.exe để chạy:
   
     Bỏ file config.cfg và AIdichdotProClientRust.exe vào chung folder với các file txt tiếng Trung cần dịch.
   
     Trong file config.cfg sẽ có "test" là username, "test1" là pasword, "True" là sử dụng phiên bản beta mới nhất, False là không sử dụng. Chạy file AIdichdotProClientRust.exe.
   
     Nếu sợ file AIdichdotProClientRust.exe có virus thì có thể tự build ra file exe bằng source code của nó từ file main.rs

3. Dùng Client.py để chạy:
   
     Cài đặt python (3.10 trở lên ). https://www.python.org/downloads/
   
     Bỏ file config.cfg và file Client.py chung folder với các file txt tiếng Trung cần dịch.
   
     Trong file config.cfg sẽ có "test" là username, "test1" là pasword, "True" là sử dụng phiên bản beta mới nhất, False là không sử dụng
   
     Double click Client.py hoặc Mở cmd (CommandPrompt) Hoặc terminal. Chuyển tới folder chứa Client.py, Gõ python client.py và nhấn Enter.
