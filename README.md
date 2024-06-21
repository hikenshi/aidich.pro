# aidich.pro

Client để chạy dịch nhiều file txt.

- Giải nén file AIdichdotProClientRust.zip.
- Bỏ các file raw tiếng trung cần dịch (các file text có đuôi là txt) vào chung với file AIdichdotProClientRust.exe.
- Chạy file AIdichdotProClientRust.exe
     + Làm theo hướng dẫn.
     + Nếu sử dụng AI để biên tập. File prompt.csv là file chứa nội dung cần hướng dẫn AI biên tập. Số chữ trong file prompt cũng bị tính phí.
          * Như file prompt.csv có 2 dòng:
            
                  Hứa Thanh là nam khoảng 21 tuổi, là nhân vật nam chính trong câu chuyện.
                  Khương Hòa là nữ, là nhân vật nữ chính trong câu chuyện.
            
          * Như vậy AI sẽ hiểu và biên tập cách xưng hô cho phù hợp.
          * Cũng có thể thêm những câu hướng dẫn như:
            
                Hứa Thanh xưng hô đối với Khương Hòa là cô.
                ...vân vân... Tùy sở thích của các bạn.
            
          * Nếu tên riêng do AI ban đầu dịch sai tên, cũng có thể hướng dẫn AI sửa cho đúng.
            
               Ví dụ:
            
                Tên riêng Tạ Vân lam bị sai, hãy sửa lại là Tạ Vân Tranh.
            
    + Xóa file config.csv để khởi tạo lại yêu cầu điền username, password và các chế độ dịch, biên tập.


- Lưu ý:
    + Nếu sử dụng AI để biên tập thì nên tách các file số lượng chữ nhiều ra làm nhiều file nhỏ, ví dụ mỗi chương 1 file, vì thời gian AI biên tập có thể rất lâu.
    + Phí sẽ được tính như sau:
      
          * AI 1 : 3 points / chữ. Chi tiết: Số chữ Trung + (Tổng số chữ đã được AIDICH dịch ra + số chữ trong file prompt.csv + prompt hệ thống [220 - 240 chữ]) x 3
          * AI 2 : 6 points / chữ. Chi tiết: Số chữ Trung + (Tổng số chữ đã được AIDICH dịch ra + số chữ trong file prompt.csv + prompt hệ thống [220 - 240 chữ]) x 6
          * AI 2 Ultra : 24 points / chữ. Chi tiết: Số chữ Trung + (Tổng số chữ đã được AIDICH dịch ra + số chữ trong file prompt.csv + prompt hệ thống [220 - 240 chữ]) x 24

    + Ví dụ:

          * Đoạn tiếng trung sau:
           北国千里冰封，万里雪景，白茫茫一片。在北凉国，苍莽群山之处，此时已经鸟兽罕见，大多数富户此时已经躲在家中，烤碳火取暖。不过在青竹县靠东的山脚下，一群少年此时却身着黑色单衣，在严寒下在一遍遍挥舞着手中的长刀。
           高台之上的一名中年武者老鹰一般锐利的目光中看着校场上试练的门中子弟。
           陆小天穿着灰色的毛皮袄，腰带上插着把柴刀，羡慕地看着校场上正在练习刀法的雷刀门弟子。
           “还不快走，磨磨蹭蹭地，今天要是没有砍到足够的柴，没有晚饭吃！”
           后面一名同样穿着灰色厚袄，中年模样的男子，脸上顾作威严的呵斥道。
           “你说什么，再说一遍试试！”此时一名身材比陆小天高过大半个头的少年在外殿门口大步走来，听到中年男子的话，冷声叱道。
           “石大哥！”
           Có số chữ trung là: 281 . Nếu dịch không dùng AI biên tập thì sẽ chỉ mất 281 point.
           Nếu dùng AI 1 biên tập thì chi phí sẽ là: 281 + (281 +  số chữ trong file prompt.csv + 225) x 3
