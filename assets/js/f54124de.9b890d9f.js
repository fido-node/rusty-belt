"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[31],{6717:(e,n,s)=>{s.r(n),s.d(n,{assets:()=>c,contentTitle:()=>i,default:()=>p,frontMatter:()=>r,metadata:()=>t,toc:()=>d});var a=s(5893),l=s(1151);s(9286);const r={sidebar_position:2,sidebar_label:"Client"},i="Client config breakout",t={id:"configuration/client-configuration",title:"Client config breakout",description:"What is handlebars?",source:"@site/docs/configuration/client-configuration.mdx",sourceDirName:"configuration",slug:"/configuration/client-configuration",permalink:"/configuration/client-configuration",draft:!1,unlisted:!1,editUrl:"https://github.com/fido-node/rusty-belt/tree/main/docs/docs/configuration/client-configuration.mdx",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2,sidebar_label:"Client"},sidebar:"tutorialSidebar",previous:{title:"Configuration",permalink:"/category/configuration"},next:{title:"Server",permalink:"/configuration/server-configuration"}},c={},d=[{value:"What is handlebars?",id:"what-is-handlebars",level:2},{value:"Segments",id:"segments",level:2},{value:"Parts",id:"parts",level:2},{value:"Session name",id:"session-name",level:3},{value:"VPN",id:"vpn",level:3},{value:"Memory",id:"memory",level:3},{value:"Swap",id:"swap",level:3},{value:"CPU",id:"cpu",level:3},{value:"Disk",id:"disk",level:3},{value:"Load average",id:"load-average",level:3},{value:"Shell",id:"shell",level:3},{value:"Color palets",id:"color-palets",level:2},{value:"Separator",id:"separator",level:2}];function o(e){const n={a:"a",br:"br",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,l.a)(),...e.components};return(0,a.jsxs)(a.Fragment,{children:[(0,a.jsx)(n.h1,{id:"client-config-breakout",children:"Client config breakout"}),"\n",(0,a.jsx)(n.h2,{id:"what-is-handlebars",children:"What is handlebars?"}),"\n",(0,a.jsxs)(n.p,{children:[(0,a.jsx)(n.a,{href:"https://handlebarsjs.com/",children:"Handlebars"})," is a simple templating language.",(0,a.jsx)(n.br,{}),"\n","Here I use ",(0,a.jsx)(n.a,{href:"https://github.com/sunng87/handlebars-rust",children:"handlebars-rust"})," implementation. I use handlebars to have a way to flexible render data structures into string templates."]}),"\n",(0,a.jsx)(n.p,{children:"Example of rich template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{children:"{{#if v}}\udb81\udd82 {{#each v}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}{{else}}No VPNs{{/if}}\n"})}),"\n",(0,a.jsxs)(n.p,{children:["You can use handlebars ",(0,a.jsx)(n.a,{href:"https://handlebarsjs.com/playground.html",children:"playground"})," to play with templates."]}),"\n",(0,a.jsx)(n.h2,{id:"segments",children:"Segments"}),"\n",(0,a.jsxs)(n.p,{children:["Segment is a building block of client config. Config may have many segments. Example config has two segments. Each segment has ",(0,a.jsx)(n.code,{children:"name"})," field. When you ask client to render something you should pass segment's name as CLI parameter ",(0,a.jsx)(n.code,{children:"--segment-name"}),".",(0,a.jsx)(n.br,{}),"\n","Each also segment have ",(0,a.jsx)(n.code,{children:"direction"})," field affects how colors applied from ",(0,a.jsx)(n.code,{children:"{fg,bg}_palette"})," and how ",(0,a.jsx)(n.code,{children:"separator"})," works.",(0,a.jsx)(n.br,{}),"\n","And the most complex part of segment is parts list."]}),"\n",(0,a.jsx)(n.h2,{id:"parts",children:"Parts"}),"\n",(0,a.jsxs)(n.p,{children:["Part mostly contains from ",(0,a.jsx)(n.code,{children:"type"})," and ",(0,a.jsx)(n.code,{children:"template"})," fields. Where type is a value from enumeration presented next and template is handlebars template string. Data structure passed to template stored in variable named ",(0,a.jsx)(n.code,{children:"v"}),". Somewhere it is single value, in other parts it may be structure.\n",(0,a.jsx)(n.code,{children:"_grpah"})," variables in ",(0,a.jsx)(n.code,{children:"v"})," is a string with a special characters which represents simple line graph analog. f.e. ",(0,a.jsx)(n.code,{children:"\u2581\u2583\u2585\u2587\u2587\u2587\u2588"})]}),"\n",(0,a.jsx)(n.p,{children:"Available part types:"}),"\n",(0,a.jsxs)(n.ul,{children:["\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#session-name",children:"session_name"})}),"\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#vpn",children:"vpn"})}),"\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#memory",children:"mem"})}),"\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#swap",children:"swap"})}),"\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#cpu",children:"cpu"})}),"\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#disk",children:"disk"})}),"\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#load-average",children:"load_average"})}),"\n",(0,a.jsx)(n.li,{children:(0,a.jsx)(n.a,{href:"#shell",children:"shell"})}),"\n"]}),"\n",(0,a.jsx)(n.h3,{id:"session-name",children:"Session name"}),"\n",(0,a.jsx)(n.p,{children:"Simple value with tmux session name in which status renders."}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"let v: String;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: session_name\ntemplate: "\uebc8: {{v}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"\uebc8: MAIN"})]}),"\n",(0,a.jsx)(n.h3,{id:"vpn",children:"VPN"}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"let v: Vec<String>;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: vpn\ntemplate: "{{#if v}}\udb81\udd82: {{#each v}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}{{else}}No VPNs connected.{{/if}}"\nnames:\n    - substring_matcher: "10.154.1."\n      name: "prod"\n    - substring_matcher: "10.154.154."\n      name: "dev"\n'})}),"\n",(0,a.jsx)(n.p,{children:"Renders to:"}),"\n",(0,a.jsxs)(n.ul,{children:["\n",(0,a.jsxs)(n.li,{children:['When you have a connected network with IP address containing "10.154.1."',(0,a.jsx)(n.br,{}),"\n",(0,a.jsx)(n.code,{children:"\udb81\udd82: prod"})]}),"\n",(0,a.jsxs)(n.li,{children:['When you have a connected networks with IP address containing "10.154.1." and "10.154.154"',(0,a.jsx)(n.br,{}),"\n",(0,a.jsx)(n.code,{children:"\udb81\udd82: prod, dev"})]}),"\n",(0,a.jsxs)(n.li,{children:["When none of above found",(0,a.jsx)(n.br,{}),"\n",(0,a.jsx)(n.code,{children:"No VPNs connected."})]}),"\n"]}),"\n",(0,a.jsx)(n.h3,{id:"memory",children:"Memory"}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"pub struct Memory {\n    pub total: String,\n    pub available: String,\n    pub used: String,\n    pub used_percents: String,\n    pub used_percents_graph: String,\n};\n\nlet v: Memory;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: mem\ntemplate: "\ue266: {{v.total}}/{{v.available}}/{{v.used}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"\ue266: 32GB/22.84GB/8.33GB"})]}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:"type: mem\ntemplate: \ue266 {{v.used_percents}}% {{v.used_percents_graph}}\n"})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"\ue266: 27% \u2583\u2583\u2583\u2583\u2583\u2583\u2583"})]}),"\n",(0,a.jsx)(n.h3,{id:"swap",children:"Swap"}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"pub struct Swap {\n    pub total: String,\n    pub used: String,\n    pub used_percents: String,\n    pub used_percents_graph: String,\n}\n\nlet v: Swap;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: swap\ntemplate: "Swap: {{v.total}}/{{v.used}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"Swap: 8GB/1.22GB"})]}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: swap\ntemplate: "\udb83\udfb4: {{v.used_percents}}% {{v.used_percents_graph}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"\udb83\udfb4: 13% \u2581\u2581\u2581\u2581\u2581\u2581\u2581\u2581"})]}),"\n",(0,a.jsx)(n.h3,{id:"cpu",children:"CPU"}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"pub struct CPU {\n    pub consumption: String,\n    pub consumption_graph: String,\n}\n\nlet v: CPU;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: cpu\ntemplate: "\uf4bc: {{v.consumption}}%"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"\uf4bc: 22%"})]}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: cpu\ntemplate: "\uf4bc: {{v.consumption}}% {{v.consumption_graph}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"\uf4bc: 99%  \u2581\u2583\u2585\u2585\u2587\u2587\u2587\u2588"})]}),"\n",(0,a.jsx)(n.h3,{id:"disk",children:"Disk"}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"pub struct DiskV {\n    pub mount_point: String,\n    pub device_path: String,\n    pub available_space: String,\n    pub total_space: String,\n    pub used_percents: String,\n}\n\nlet v: Disk;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: disk\ndev: /dev/mapper/luks-7a504a5c-****-****-****-3a97c15a21ce\ntemplate: "\uf0a0  {{v.mount_point}} {{v.available_space}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"\uf0a0  / 623.26GB"})]}),"\n",(0,a.jsx)(n.h3,{id:"load-average",children:"Load average"}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"pub struct LA {\n    pub one: String,\n    pub five: String,\n    pub fifteen: String,\n}\n\nlet v: LA;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: load_average\ntemplate: "LA: {{v.one}}, {{v.five}}, {{v.fifteen}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"LA: 1.01, 1.00, 0.93"})]}),"\n",(0,a.jsx)(n.h3,{id:"shell",children:"Shell"}),"\n",(0,a.jsx)(n.p,{children:"Data payload for template:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-rust",children:"pub struct Shell {\n    pub stdout: String,\n}\n\nlet v: Shell;\n"})}),"\n",(0,a.jsx)(n.p,{children:"Example:"}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-yaml",children:'type: shell\ncmd: free -m -h | awk \'/Mem/{printf $3"/"$2}\'\ntemplate: "{{v.stdout}}"\n'})}),"\n",(0,a.jsxs)(n.p,{children:["Renders to: ",(0,a.jsx)(n.code,{children:"8,5Gi/31Gi"})," (Result of command)"]}),"\n",(0,a.jsx)(n.h2,{id:"color-palets",children:"Color palets"}),"\n",(0,a.jsxs)(n.p,{children:["There are color palette in each segment. It may be empty or it may be list of HEX colors. If your parts list is longer than palette, clinet will use palette again and again.\nIf ",(0,a.jsx)(n.code,{children:"direction"})," set to ",(0,a.jsx)(n.code,{children:"ltr"}),", client will reverse palette. So basically you can use same palette in left and right segments, but two segments will consume palette from different sides."]}),"\n",(0,a.jsx)(n.h2,{id:"separator",children:"Separator"}),"\n",(0,a.jsxs)(n.p,{children:["With separator you can replicate powerline theme. Just set symbol you want to see as separator and set correct direction for segment. Exapmple config has standart powerline separators ",(0,a.jsx)(n.code,{children:"\ue0b0"})]})]})}function p(e={}){const{wrapper:n}={...(0,l.a)(),...e.components};return n?(0,a.jsx)(n,{...e,children:(0,a.jsx)(o,{...e})}):o(e)}}}]);