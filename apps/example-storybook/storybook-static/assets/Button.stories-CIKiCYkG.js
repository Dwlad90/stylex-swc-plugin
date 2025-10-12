import{j as V}from"./jsx-runtime-ByyrC_Ko.js";import"./iframe-CRcfLhXq.js";import"./preload-helper-PPVm8Dsz.js";var x={},P;function G(){if(P)return x;P=1,Object.defineProperty(x,"__esModule",{value:!0}),x.styleq=void 0;var d=new WeakMap,o="$$css";function n(e){var _,D,M;return e!=null&&(_=e.disableCache===!0,D=e.disableMix===!0,M=e.transform),function(){for(var c=[],b="",a=null,m="",t=_?null:d,k=new Array(arguments.length),y=0;y<arguments.length;y++)k[y]=arguments[y];for(;k.length>0;){var i=k.pop();if(!(i==null||i===!1)){if(Array.isArray(i)){for(var $=0;$<i.length;$++)k.push(i[$]);continue}var r=M!=null?M(i):i;if(r.$$css!=null){var l="";if(t!=null&&t.has(r)){var f=t.get(r);f!=null&&(l=f[0],m=f[2],c.push.apply(c,f[1]),t=f[3])}else{var H=[];for(var u in r){var g=r[u];if(u===o){var w=r[u];w!==!0&&(m=m?w+"; "+m:w);continue}typeof g=="string"||g===null?c.includes(u)||(c.push(u),t!=null&&H.push(u),typeof g=="string"&&(l+=l?" "+g:g)):console.error("styleq: ".concat(u," typeof ").concat(String(g),' is not "string" or "null".'))}if(t!=null){var O=new WeakMap;t.set(r,[l,H,m,O]),t=O}}l&&(b=b?l+" "+b:l)}else if(D)a==null&&(a={}),a=Object.assign({},r,a);else{var p=null;for(var v in r){var C=r[v];C!==void 0&&(c.includes(v)||(C!=null&&(a==null&&(a={}),p==null&&(p={}),p[v]=C),c.push(v),t=null))}p!=null&&(a=Object.assign(p,a))}}}var A=[b,a,m];return A}}var s=x.styleq=n();return s.factory=n,x}var R=G();function I(...d){const[o,n,s]=R.styleq(d),e={};return o!=null&&o!==""&&(e.className=o),n!=null&&Object.keys(n).length>0&&(e.style=n),s!=null&&s!==""&&(e["data-style-src"]=s),e}const T={base:{"fontFamily-kMv6JI":"fontFamily-xmcqa5u","fontWeight-k63SB2":"fontWeight-xk50ysn","cursor-kkrTdU":"cursor-x1ypdohk","borderWidth-kMzoRj":"borderWidth-xc342km","borderStyle-ksu8eU":"borderStyle-xng3xce","borderColor-kVAM5u":"borderColor-x9r1u3d","borderRadius-kaIpWk":"borderRadius-x1kogg8i","transition-kmkexE":"transition-xurpv0g","display-k1xSpc":"display-x3nfvp2","alignItems-kGNEyG":"alignItems-x6s0dn4","justifyContent-kjj79g":"justifyContent-xl56j7k","textDecoration-kybGjl":"textDecoration-x1hl2dhg","transform-k3aq6I":"transform-xnn1q72 transform-x14ow2ja transform-x3kbueh","boxShadow-kGVxlE":"boxShadow-x1gnnqk1 boxShadow-x1yrdse8",$$css:"stories/Button.tsx:5"},primary:{"backgroundColor-kWkggS":"backgroundColor-x11jye9f backgroundColor-xkdfp31","color-kMwMTN":"color-x1awj2ng",$$css:"stories/Button.tsx:28"},secondary:{"backgroundColor-kWkggS":"backgroundColor-x6cgz0f backgroundColor-x1jmbrsh","color-kMwMTN":"color-x1525slw","borderWidth-kMzoRj":"borderWidth-xmkeg23","borderStyle-ksu8eU":"borderStyle-x1y0btm7","borderColor-kVAM5u":"borderColor-x7kbyhx",$$css:"stories/Button.tsx:35"},danger:{"backgroundColor-kWkggS":"backgroundColor-x1596o1h backgroundColor-x37qbrc","color-kMwMTN":"color-x1awj2ng",$$css:"stories/Button.tsx:45"},small:{"fontSize-kGuDYH":"fontSize-xfifm61","padding-kmVPX3":"padding-x1arfzav","minHeight-kAzted":"minHeight-xe0p6wg",$$css:"stories/Button.tsx:52"},medium:{"fontSize-kGuDYH":"fontSize-xif65rj","padding-kmVPX3":"padding-x1ff1495","minHeight-kAzted":"minHeight-xu0aao5",$$css:"stories/Button.tsx:57"},large:{"fontSize-kGuDYH":"fontSize-x1j61zf2","padding-kmVPX3":"padding-x1bg5miv","minHeight-kAzted":"minHeight-x1gg8mnh",$$css:"stories/Button.tsx:62"}},W=({size:d="medium",variant:o="primary",label:n,onClick:s})=>V.jsx("button",{type:"button",...I(T.base,T[d],T[o]),onClick:s,children:n});try{W.displayName="Button",W.__docgenInfo={description:"",displayName:"Button",props:{size:{defaultValue:{value:"medium"},description:"The size of the button",name:"size",required:!1,type:{name:"enum",value:[{value:'"medium"'},{value:'"small"'},{value:'"large"'}]}},variant:{defaultValue:{value:"primary"},description:"The variant of the button",name:"variant",required:!1,type:{name:"enum",value:[{value:'"primary"'},{value:'"secondary"'},{value:'"danger"'}]}},label:{defaultValue:null,description:`The label of the button
@example 'Click me'`,name:"label",required:!0,type:{name:"string"}},onClick:{defaultValue:null,description:"Function to call when the button is clicked",name:"onClick",required:!1,type:{name:"(() => void)"}}}}}catch{}const{fn:E}=__STORYBOOK_MODULE_TEST__,N={title:"Example/Button",component:W,parameters:{layout:"centered"},tags:["autodocs"],args:{onClick:E()},argTypes:{variant:{control:{type:"select"},options:["primary","secondary","danger"],description:"The variant of the button"},size:{control:{type:"select"},options:["small","medium","large"]}}},h={args:{variant:"primary",label:"Primary Button"}},S={args:{variant:"secondary",label:"Secondary Button"}},B={args:{variant:"danger",label:"Danger Button"}},z={args:{size:"small",label:"Small Button"}},j={args:{size:"medium",label:"Medium Button"}},q={args:{size:"large",label:"Large Button"}};h.parameters={...h.parameters,docs:{...h.parameters?.docs,source:{originalSource:`{
  args: {
    variant: 'primary',
    label: 'Primary Button'
  }
}`,...h.parameters?.docs?.source}}};S.parameters={...S.parameters,docs:{...S.parameters?.docs,source:{originalSource:`{
  args: {
    variant: 'secondary',
    label: 'Secondary Button'
  }
}`,...S.parameters?.docs?.source}}};B.parameters={...B.parameters,docs:{...B.parameters?.docs,source:{originalSource:`{
  args: {
    variant: 'danger',
    label: 'Danger Button'
  }
}`,...B.parameters?.docs?.source}}};z.parameters={...z.parameters,docs:{...z.parameters?.docs,source:{originalSource:`{
  args: {
    size: 'small',
    label: 'Small Button'
  }
}`,...z.parameters?.docs?.source}}};j.parameters={...j.parameters,docs:{...j.parameters?.docs,source:{originalSource:`{
  args: {
    size: 'medium',
    label: 'Medium Button'
  }
}`,...j.parameters?.docs?.source}}};q.parameters={...q.parameters,docs:{...q.parameters?.docs,source:{originalSource:`{
  args: {
    size: 'large',
    label: 'Large Button'
  }
}`,...q.parameters?.docs?.source}}};const X=["Primary","Secondary","Danger","Small","Medium","Large"];export{B as Danger,q as Large,j as Medium,h as Primary,S as Secondary,z as Small,X as __namedExportsOrder,N as default};
