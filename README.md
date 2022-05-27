<p align="center">
<img src="samples/large.jpg" alt="large"/>
<table>
<tr>
<td><img src="samples/out1.jpg" alt="out1"/></td>
<td><img src="samples/out2.jpg" alt="out2"/></td>
<td><img src="samples/out3.jpg" alt="out3"/></td>
<td><img src="samples/out4.jpg" alt="out4"/></td>
<td><img src="samples/out5.jpg" alt="out5"/></td>
<td><img src="samples/out6.jpg" alt="out6"/></td>
<td><img src="samples/out7.jpg" alt="out7"/></td>
<td><img src="samples/out8.jpg" alt="out8"/></td>
<td><img src="samples/out9.jpg" alt="out9"/></td>
</tr>
</table>
</p>

Usage:

```
cargo run --release <width> <height> <input-image> [output-image]
```

Example:

```
cargo run --release 200 200 monet.jpg /tmp/out.png
```

Inspired by https://codegolf.stackexchange.com/a/22326.