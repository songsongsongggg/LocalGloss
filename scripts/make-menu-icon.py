#!/usr/bin/env python3
"""生成自己的简单矢量菜单图标，不使用上游 logo。"""
import pathlib
import sys

stream=b'BT /F1 11 Tf 0 0 0 rg 0 3 Td (LG) Tj ET\n'
objects=[b'<< /Type /Catalog /Pages 2 0 R >>',
         b'<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
         b'<< /Type /Page /Parent 2 0 R /MediaBox [0 0 18 18] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>',
         b'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>',
         b'<< /Length '+str(len(stream)).encode()+b' >>\nstream\n'+stream+b'endstream']
data=bytearray(b'%PDF-1.4\n')
offsets=[0]
for index,obj in enumerate(objects,1):
    offsets.append(len(data))
    data.extend(str(index).encode()+b' 0 obj\n'+obj+b'\nendobj\n')
xref=len(data)
data.extend(b'xref\n0 6\n0000000000 65535 f \n')
for offset in offsets[1:]: data.extend(f'{offset:010d} 00000 n \n'.encode())
data.extend(f'trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n'.encode())
pathlib.Path(sys.argv[1]).write_bytes(data)
