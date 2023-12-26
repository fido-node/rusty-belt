"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[507],{4022:(e,n,s)=>{s.r(n),s.d(n,{assets:()=>p,contentTitle:()=>d,default:()=>g,frontMatter:()=>c,metadata:()=>u,toc:()=>h});var t=s(5893),r=s(1151),i=s(9286);const a="[Unit]\nDescription=Rusty-belt server part\nDocumentation=https://github.com/fido-node/rusty-belt\n\n[Service]\nType=simple\nExecStart=%h/.cargo/bin/rusty_belt_server\nKillMode=process\nRestart=on-failure\nUser\n\n[Install]\nWantedBy=default.target\n",l="---\nserver:\n  update_interval: 2\nsegments:\n  - name: left\n    bg_palet:\n      - f5e0dc\n      - f5c2e7\n      - cba6f7\n      - eba0ac\n      - f9e2af\n      - a6e3a1\n      - 94e2d5\n      - 89dceb\n      - 89b4fa\n      - b4befe\n    fg_palet:\n      - 11111b\n      - 181825\n      - 1e1e2e\n    separator: \ue0b0\n    parts:\n      - type: session_name\n        # Where v is string value with session name\n        template: \"\uebc8: {{v}}\"\n      - type: vpn\n        # Where v is a Vec of strings\n        template: \"{{#if v}}\udb81\udd82: {{#each v}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}{{else}}No VPNs connected.{{/if}}\"\n        names:\n          - substring_matcher: \"10.154.1.\"\n            name: \"prod\"\n          - substring_matcher: \"10.154.154.\"\n            name: \"dev\"\n      - type: mem\n        # Where v is a structure with humanized string fields 'used', 'available', 'total' and 'used_percents'\n        template: \"\ue266: {{v.total}}/{{v.available}}/{{v.used}}\"\n      - type: mem\n        # Where v is a structure with humanized string fields 'used', 'available', 'total' and 'used_percents'\n        template: \"Mem: {{v.used_percents}}% {{v.used_percents_graph}}\"\n      - type: cpu\n        # Where v is a string value without percent symbol\n        template: \"\uf4bc: {{v.consumption}}%\"\n      - type: load_average\n        # Where v is a structure with humanized string fields 'one', 'five' and 'fifteen'\n        template: \"LA: {{v.one}}, {{v.five}}, {{v.fifteen}}\"\n      - type: swap\n        # Where v is a structure with humanized string fields 'used', 'total' and 'used_percents'\n        template: \"Swap: {{v.total}}/{{v.used}}\"\n      - type: swap\n        # Where v is a structure with humanized string fields 'used', 'total' and 'used_percents'\n        template: \"\udb83\udfb4: {{v.used_percents}}%\"\n      - type: disk\n        dev: /dev/mapper/luks-7a504a5c-d5f0-4175-95b6-3a97c15a21ce\n        # Where v is a structure with\n        template: \"\uf0a0 {{v.mount_point}} {{v.used_percents}}%\"\n      - type: shell\n        cmd: free -m -h | awk '/Mem/{printf $3\"/\"$2}'\n        template: \"{{v.stdout}}\"\n  - name: right\n    bg_palet:\n      - f5e0dc\n      - f5c2e7\n      - cba6f7\n      - eba0ac\n      - f9e2af\n      - a6e3a1\n      - 94e2d5\n      - 89dceb\n      - 89b4fa\n      - b4befe\n    fg_palet:\n      - 11111b\n      - 181825\n      - 1e1e2e\n    separator: \ue0b2\n    direction: rtl\n    parts:\n      - type: shell\n        use_pwd: true\n        cmd: gitmux -cfg ~/.config/tmux/gitmux.yaml\n        template: \"{{v.stdout}}\"\n",o="---\nrefresh_rate: 30 seconds\nappenders:\n  stdout:\n    kind: console\nroot:\n  level: debug\n  appenders:\n    - stdout\n",c={sidebar_position:2,sidebar_label:"Linux"},d=void 0,u={id:"installation/Linux",title:"Linux",description:"Requirements",source:"@site/docs/installation/Linux.mdx",sourceDirName:"installation",slug:"/installation/Linux",permalink:"/installation/Linux",draft:!1,unlisted:!1,editUrl:"https://github.com/fido-node/rusty-belt/tree/main/docs/docs/installation/Linux.mdx",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2,sidebar_label:"Linux"},sidebar:"tutorialSidebar",previous:{title:"Installation",permalink:"/category/installation"},next:{title:"Configuration",permalink:"/category/configuration"}},p={},h=[{value:"Requirements",id:"requirements",level:2},{value:"Cargo",id:"cargo",level:3},{value:"Build from source",id:"build-from-source",level:3},{value:"Run as a service",id:"run-as-a-service",level:3},{value:"Use sample configs",id:"use-sample-configs",level:3}];function m(e){const n={a:"a",br:"br",code:"code",h2:"h2",h3:"h3",p:"p",pre:"pre",strong:"strong",...(0,r.a)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h2,{id:"requirements",children:"Requirements"}),"\n",(0,t.jsxs)(n.p,{children:["You need to install rustc and cargo. Yo can use ",(0,t.jsx)(n.strong,{children:"rustup"})," or your package manager for your system."]}),"\n",(0,t.jsx)(n.h3,{id:"cargo",children:"Cargo"}),"\n",(0,t.jsxs)(n.p,{children:["Install with cargo from ",(0,t.jsx)(n.strong,{children:"crates.io"})]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"cargo install rusty-belt\n"})}),"\n",(0,t.jsx)(n.h3,{id:"build-from-source",children:"Build from source"}),"\n",(0,t.jsx)(n.p,{children:"Clone repository"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"git clone https://github.com/fido-node/rusty-belt.git\n"})}),"\n",(0,t.jsx)(n.p,{children:"Build binaries with release flag"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"cargo build --release\n"})}),"\n",(0,t.jsxs)(n.p,{children:["Move binaries from ",(0,t.jsx)(n.code,{children:"./target/release/"})," to some dir in your ",(0,t.jsx)(n.code,{children:"PATH"})]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"cp ./target/release/tmux_client ~/.local/bin\ncp ./target/release/rusty_belt_server ~/.local/bin\n"})}),"\n",(0,t.jsx)(n.h3,{id:"run-as-a-service",children:"Run as a service"}),"\n",(0,t.jsxs)(n.p,{children:["You can use this file ",(0,t.jsx)(n.a,{href:"https://github.com/fido-node/rusty-belt/blob/main/resources/rusty-belt.service",children:(0,t.jsx)(n.strong,{children:"rusty-belt.service"})})," for example:"]}),"\n",(0,t.jsxs)(n.p,{children:["Fix 7th line acording to the way you've installed binaries.",(0,t.jsx)(n.br,{}),"\n","Use ",(0,t.jsx)(n.code,{children:"/.cargo/bin/rusty_belt_server"})," if you choose cargo as source of binaries.",(0,t.jsx)(n.br,{}),"\n","Use ",(0,t.jsx)(n.code,{children:"/.local/bin/rusty_belt_server"})," if you build it from sources."]}),"\n",(0,t.jsx)(i.Z,{language:"systemd",title:"~/.config/systemd/user/rusty-belt.service",showLineNumbers:!0,children:a}),"\n",(0,t.jsxs)(n.p,{children:["Place it to the ",(0,t.jsx)(n.code,{children:"~/.config/systemd/user/"})]}),"\n",(0,t.jsx)(n.p,{children:"Reload services, enable and start it."}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"systemctl daemon-reload --user\nsystemctl --user enable rusty-belt.service\nsystemctl --user start rusty-belt.service\n"})}),"\n",(0,t.jsx)(n.h3,{id:"use-sample-configs",children:"Use sample configs"}),"\n",(0,t.jsxs)(n.p,{children:["Use sample configs or go to ",(0,t.jsx)(n.a,{href:"../category/configuration",children:(0,t.jsx)(n.strong,{children:"configuration doc"})})]}),"\n",(0,t.jsxs)(n.p,{children:["Place these files to ",(0,t.jsx)(n.code,{children:"~/.config/rusty-belt/"})]}),"\n",(0,t.jsx)(n.p,{children:"Sample app config:"}),"\n",(0,t.jsx)(i.Z,{language:"yaml",title:"~/.config/rusty-belt/config.yaml",showLineNumbers:!0,children:l}),"\n",(0,t.jsx)(n.p,{children:"Sample logging config:"}),"\n",(0,t.jsx)(i.Z,{language:"yaml",title:"~/.config/rusty-belt/log4rs.yaml",showLineNumbers:!0,children:o})]})}function g(e={}){const{wrapper:n}={...(0,r.a)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(m,{...e})}):m(e)}}}]);