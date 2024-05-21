# aidich.pro

Client để chạy dịch nhiều file txt.

- Giải nén file AIdichdotProClientRust.zip.
- Bỏ các file raw tiếng trung cần dịch (các file text có đuôi là txt) vào chung với file AIdichdotProClientRust.exe.
- Chạy file AIdichdotProClientRust.exe
     + Làm theo hướng dẫn.
     + Nếu sử dụng Gemini để biên tập. File prompt.csv là file chứa nội dung cần hướng dẫn Gemini biên tập. Số chữ trong file prompt cũng bị tính phí.
          * Như file prompt.csv có 2 dòng:
            
               *Hứa Thanh là nam khoảng 21 tuổi, là nhân vật nam chính trong câu chuyện.\n
               *Khương Hòa là nữ, là nhân vật nữ chính trong câu chuyện.
            
          * Như vậy Gemini sẽ hiểu và biên tập cách xưng hô cho phù hợp.
          * Cũng có thể thêm những câu hướng dẫn như:
            
               Hứa Thanh xưng hô đối với Khương Hòa là cô.
               ...vân vân... Tùy sở thích của các bạn.
            
          * Nếu tên riêng do AI ban đầu dịch sai tên, cũng có thể hướng dẫn Gemini sửa cho đúng.
            
               Ví dụ: Tên riêng Tạ Vân lam bị sai, hãy sửa lại là Tạ Vân Tranh.
            
    + Xóa file config.csv để khởi tạo lại yêu cầu điền username, password và các chế độ dịch, biên tập.


- Lưu ý:
    + Nếu sử dụng Gemini để biên tập thì nên tách các file số lượng chữ nhiều ra làm nhiều file nhỏ, ví dụ mỗi chương 1 file, vì thời gian Gemini biên tập có thể rất lâu.
    + Phí sẽ được tính như sau:
      
          * Gemini 1.0 : 5 points / chữ. Chi tiết: (Tổng số chữ đã được AIDICH dịch ra + số chữ trong file prompt.csv + prompt hệ thống [200 chữ]) x 5
          * Gemini 1.5 : 12 points / chữ. Chi tiết: (Tổng số chữ đã được AIDICH dịch ra + số chữ trong file prompt.csv + prompt hệ thống [200 chữ]) x 12
