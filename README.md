# Bouncing window manager

Julia Evans window manager [challenge].

[challenge][https://jvns.ca/blog/2019/11/25/challenge--make-a-bouncy-window-manager/]

```bash
git clone https://github.com/light4/bouncywm.git
cargo build --release
# 启动新的 Xserver 来测试, 不影响现有服务
Xephyr -ac -screen 1280x1024 -br -reset -terminate 2> /dev/null :1 &
# 在新的 Xserver 中启动 xterm
xterm -display :1
# 在新的 Xserver 中启动 bouncywm
env DISPLAY=:1 ./target/release/bouncywm
# 按 Ctrl-E 开始弹跳
```
