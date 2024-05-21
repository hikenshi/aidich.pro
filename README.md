# aidich.pro

Client để chạy dịch nhiều file txt.

- Giải nén file AIdichdotProClientRust.zip.
- Chạy file AIdichdotProClientRust.exe
     + Làm theo hướng dẫn.
     + Nếu sử dụng Gemini để biên tập. File prompt.csv là file chứa nội dung cần hướng dẫn Gemini biên tập. Số chữ trong file prompt cũng bị tính phí.
          * Như file prompt.csv có 2 dòng:
               Hứa Thanh là nam khoảng 21 tuổi, là nhân vật nam chính trong câu chuyện.
               Khương Hòa là nữ, là nhân vật nữ chính trong câu chuyện.
          * Như vậy Gemini sẽ hiểu và biên tập cách xưng hô cho phù hợp.
          * Cũng có thể thêm những câu hướng dẫn như:
               Hứa Thanh xưng hô đối với Khương Hòa là cô.
               ...vân vân... Tùy sở thích của các bạn.
          * Nếu tên riêng do AI ban đầu dịch sai tên, cũng có thể hướng dẫn Gemini sửa cho đúng.
               Ví dụ: Tên riêng Tạ Vân lam bị sai, hãy sửa lại là Tạ Vân Tranh.
    + Xóa file config.csv để khởi tạo lại yêu cầu điền username, password và các chế độ dịch, biên tập.
