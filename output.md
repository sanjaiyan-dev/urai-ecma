# Project Title: rspress-plugin-third-parties 

## Project Description

Optimized third-party components (YouTube, Google Maps, Twitter/X, GA) and script loading strategies for Rspress.

Dependencies used in this project: 
   - **react-compiler-runtime** : `0.0.0-experimental-a1856f3-20260507`
   - **react-tweet** : `^3.3.1`
   - **third-party-capital** : `^3.0.0`

Dev dependencies used in this project: 
   - **@biomejs/biome** : `2.5.5`
   - **@changesets/cli** : `^2.31.1`
   - **@rsbuild/plugin-react** : `^2.1.0`
   - **@rslib/core** : `^0.23.2`
   - **@rspress/core** : `^2.0.18`
   - **@rspress/shared** : `^2.0.18`
   - **@types/node** : `^26.1.1`
   - **@types/react** : `^19.2.17`
   - **@types/react-dom** : `^19.2.3`
   - **babel-plugin-react-compiler** : `0.0.0-experimental-a1856f3-20260507`
   - **gh-pages** : `^6.3.0`
   - **react** : `^19.2.8`
   - **react-dom** : `^19.2.8`
   - **typescript** : `^7.0.2`

#### Project Version: 2.1.1 


---

## Project File Structure & PEG Graph

```
📁 rspress-plugin-third-parties
├── CHANGELOG.md
├── LICENSE
├── Readme.md
├── biome.json
├── examples/
│   └── demo/
│       ├── doc_build/
│       │   ├── 404.html
│       │   ├── demo.html
│       │   ├── demo.md
│       │   ├── index.html
│       │   ├── index.md
│       │   ├── llms-full.txt
│       │   ├── llms.txt
│       │   ├── readme.html
│       │   ├── readme.md
│       │   ├── rspress-plugin-third-party-hero.png
│       │   ├── rspress-plugin-third-party-icon.png
│       │   └── static/
│       │       ├── css/
│       │       │   └── styles.dbac63abca.css
│       │       ├── js/
│       │       │   ├── 446.f8fecbd06e.js
│       │       │   ├── 446.f8fecbd06e.js.LICENSE.txt
│       │       │   ├── async/
│       │       │   │   ├── 147.5e325c50c1.js
│       │       │   │   ├── 147.5e325c50c1.js.LICENSE.txt
│       │       │   │   ├── 338.b72b9ddc0e.js
│       │       │   │   ├── 822.94e475284c.js
│       │       │   │   ├── route-2308615ee227.bddbb04ec6.js
│       │       │   │   ├── route-803c96454bcf.dda240fa67.js
│       │       │   │   └── route-f0296560b05f.db882ed34d.js
│       │       │   ├── index.2d81cf6c70.js
│       │       │   ├── lib-react.b8456f9435.js
│       │       │   ├── lib-react.b8456f9435.js.LICENSE.txt
│       │       │   ├── lib-router.bfd57fec2a.js
│       │       │   ├── lib-router.bfd57fec2a.js.LICENSE.txt
│       │       │   └── styles.5a599e8897.js
│       │       └── search_index.en.56b50b1f.json
│       ├── docs/
│       │   ├── demo.mdx
│       │   ├── index.mdx
│       │   ├── public/
│       │   │   ├── rspress-plugin-third-party-hero.png
│       │   │   └── rspress-plugin-third-party-icon.png
│       │   └── readme.mdx
│       ├── package.json
│       ├── rspress.config.ts
│       └── tsconfig.json
├── package.json
├── pnpm-lock.yaml
├── pnpm-workspace.yaml
├── rslib.config.ts
├── src/
│   ├── components/
│   │   ├── GoogleAnalytics/
│   │   │   ├── GlobalGoogleAnalytics.tsx
│   │   │   └── index.tsx
│   │   ├── GoogleMapsEmbed.tsx
│   │   ├── GoogleTagManager.tsx
│   │   ├── Script.tsx
│   │   ├── ThirdPartyScripts.tsx
│   │   ├── TweetEmbed.tsx
│   │   ├── YouTubeEmbed.tsx
│   │   └── utils.ts
│   ├── index.ts
│   └── plugin.ts
└── tsconfig.json
```

### Module Dependency Graph

```mermaid
graph TD;
    "src/index.ts" --> "./components/GoogleAnalytics/index";
    "src/index.ts" --> "./components/GoogleMapsEmbed";
    "src/index.ts" --> "./components/GoogleTagManager";
    "src/index.ts" --> "./components/Script";
    "src/index.ts" --> "./components/TweetEmbed";
    "src/index.ts" --> "./components/YouTubeEmbed";
    "src/components/YouTubeEmbed.tsx" --> "./Script";
    "src/components/YouTubeEmbed.tsx" --> "./Script";
    "src/components/YouTubeEmbed.tsx" --> "./ThirdPartyScripts";
    "src/components/GoogleAnalytics/index.tsx" --> "../Script";
    "src/components/GoogleAnalytics/GlobalGoogleAnalytics.tsx" --> "./index";
    "src/components/Script.tsx" --> "./utils";
    "src/components/GoogleMapsEmbed.tsx" --> "./ThirdPartyScripts";
    "src/components/GoogleTagManager.tsx" --> "./Script";
```

---

## React Component Architecture & Explanations

### React Component Breakdown: `<YouTubeEmbed>` 

- **Props**:
  - `props` (type: `YouTubeEmbedTypes`)
- **State**: Stateless component.
- **Rendered JSX Tree**: `<ThirdPartyScriptEmbed>, <Script>` 

### React Component Breakdown: `<GoogleAnalytics>` 

- **Props**:
  - `props` (type: `GAParams`)
- **State**: Stateless component.
- **Rendered JSX Tree**: `<Script>` 

### React Component Breakdown: `<Script>` 

- **Props**:
  - `props` (type: `ScriptProps`)
- **State**: Stateless component.
- **Hooks**: Uses `useRef, useEffect` (Total Side-Effects: 2).
- **Rendered JSX Tree**: `<script>` 

### React Component Breakdown: `<GoogleMapsEmbed>` 

- **Props**:
  - `props` (type: `GoogleMapsEmbedTypes`)
- **State**: Stateless component.
- **Rendered JSX Tree**: `<ThirdPartyScriptEmbed>` 

### React Component Breakdown: `<GoogleTagManager>` 

- **Props**:
  - `props` (type: `GTMParams`)
- **State**: Stateless component.
- **Rendered JSX Tree**: `<Script>` 

### React Component Breakdown: `<TweetEmbed>` 

- **Props**:
  - `props` (type: `TweetEmbedTypes`)
- **State**: Stateless component.
- **Hooks**: Uses `useDark` (Total Side-Effects: 0).
- **Rendered JSX Tree**: `<figure>, <Tweet>, <figcaption>` 

---

## AST-Pruned Source Code Repository

> Note: Tailwind classNames and static styles have been pruned according to mode to maximize token efficiency.

### File: `rslib.config.ts`

```typescript
import { pluginReact } from "@rsbuild/plugin-react";
import { defineConfig } from "@rslib/core";
export default defineConfig({
    source: {
        entry: {
            index: "./src/index.ts",
            plugin: "./src/plugin.ts",
            GlobalGoogleAnalytics: "./src/components/GoogleAnalytics/GlobalGoogleAnalytics.tsx"
        }
    },
    lib: [
        {
            format: "esm",
            syntax: "es2022",
            dts: true
        },
        {
            format: "cjs",
            syntax: "es2022",
            dts: true
        }
    ],
    plugins: [
        pluginReact({
            reactCompiler: {
                target: "18"
            }
        })
    ]
});

```

### File: `src/index.ts`

```typescript
export * from "./components/GoogleAnalytics/index";
export * from "./components/GoogleMapsEmbed";
export * from "./components/GoogleTagManager";
export * from "./components/Script";
export * from "./components/TweetEmbed";
export * from "./components/YouTubeEmbed";

```

### File: `src/components/YouTubeEmbed.tsx`

**Function Summaries**:
- Line 40: `YouTubeEmbed` -> *Renders an embedded YouTube video using optimized scripts and stylesheets derived from passed properties.*

```typescript
import ReactDOM from "react-dom";
import { YouTubeEmbed as TPCYouTubeEmbed } from "third-party-capital";
import type { ScriptProps } from "./Script";
import { Script } from "./Script";
import ThirdPartyScriptEmbed from "./ThirdPartyScripts";
export type YouTubeEmbedTypes = {
    height?: number;
    width?: number;
    videoid: string;
    playlabel?: string;
    params?: string;
    style?: string;
};
interface PreconnectOptions {
    crossOrigin?: "anonymous" | "use-credentials" | "";
}
const safePreconnect = (href: string, options?: PreconnectOptions)=>{
    const preconnectFn = (ReactDOM as any).preconnect || (ReactDOM as any).experimental_preconnect;
    if (typeof preconnectFn === "function") {
        preconnectFn(href, options);
    }
};
export const scriptStrategy = {
    server: "beforeInteractive",
    client: "afterInteractive",
    idle: "lazyOnload",
    worker: "lazyOnload"
} as const;
const youtubePreconnectOpts = {
    crossOrigin: ""
} as const;
export function YouTubeEmbed(props: YouTubeEmbedTypes) {
    const { html, scripts, stylesheets } = TPCYouTubeEmbed(props);
    safePreconnect("https://cdn.jsdelivr.net", youtubePreconnectOpts);
    return (<ThirdPartyScriptEmbed height={props.height || null} width={props.width || null} html={html}>
			{scripts?.flatMap((script)=>{
        if (script && "url" in script && typeof script.url === "string") {
            return [
                <Script key={script.key ?? script.url} src={script.url} strategy={scriptStrategy[script.strategy as keyof typeof scriptStrategy] as ScriptProps["strategy"]} stylesheets={stylesheets}/>
            ];
        }
        return [];
    })}
		</ThirdPartyScriptEmbed>);
}

```

### File: `src/components/GoogleAnalytics/index.tsx`

**Function Summaries**:
- Line 14: `GoogleAnalytics` -> *Initializes Google Analytics tracking by injecting a data layer script and loading the necessary gtag.js library using the provided GA ID.*
- Line 44: `sendGAEvent` -> *Dispatches provided event arguments into a specific Google Analytics data layer on the global window object, but only if that data layer has been successfully initialized.*

```typescript
"use client";
import { Script } from "../Script";
export type GAParams = {
    gaId: string;
    dataLayerName?: string;
    debugMode?: boolean;
    nonce?: string;
};
let currDataLayerName: string | undefined;
export function GoogleAnalytics(props: GAParams) {
    const { gaId, debugMode, dataLayerName = "dataLayer", nonce } = props;
    if (currDataLayerName === undefined) {
        currDataLayerName = dataLayerName;
    }
    return (<>
			<Script id="_rspress-ga-init" dangerouslySetInnerHTML={{
        __html: `
            window['${dataLayerName}'] = window['${dataLayerName}'] || [];
            function gtag(){window['${dataLayerName}'].push(arguments);}
            gtag('js', new Date());
            gtag('config', '${gaId}' ${debugMode ? ",{ 'debug_mode': true }" : ""});
          `
    }} nonce={nonce}/>
			<Script id="_rspress-ga" src={`https://www.googletagmanager.com/gtag/js?id=${gaId}`} nonce={nonce}/>
		</>);
}
export function sendGAEvent(..._args: any[]) {
    if (currDataLayerName === undefined) {
        console.warn(`Rspress Third Parties: GA has not been initialized`);
        return;
    }
    const win = window as any;
    if (win[currDataLayerName]) {
        win[currDataLayerName].push(arguments);
    } else {
        console.warn(`Rspress Third Parties: GA dataLayer "${currDataLayerName}" does not exist`);
    }
}

```

### File: `src/components/GoogleAnalytics/GlobalGoogleAnalytics.tsx`

```typescript
import { GoogleAnalytics } from "./index";
export default function GlobalGoogleAnalytics() {
    const configString = process.env.RSPRESS_GA_CONFIG;
    if (!configString || configString === "null") {
        return null;
    }
    try {
        const config = typeof configString === "string" ? JSON.parse(configString) : configString;
        if (!config || !config.gaId) {
            return null;
        }
        return <GoogleAnalytics {...config}/>;
    } catch (err) {
        console.error("[rspress-plugin-third-parties] Failed to parse GA config:", err);
        return null;
    }
}

```

### File: `src/components/Script.tsx`

**Function Summaries**:
- Line 182: `handleClientScriptLoad` -> *Loads a client script using different timing strategies, executing immediately by default or deferring execution until the browser is idle after the page has fully loaded if "lazyOnload" is specified.*
- Line 193: `loadLazyScript` -> *Ensures a script is loaded asynchronously using `requestIdleCallback` either immediately if the document is ready or upon the window's load event.*
- Line 203: `addBeforeInteractiveToCache` -> *Identifies all DOM elements marked with the 'beforeInteractive' attribute and registers their source URLs into a global application cache for tracking or loading management.*
- Line 224: `Script` -> *Loads an external or internal script resource into the DOM based on specified loading strategies and handles stylesheet preloading for optimal performance.*

```typescript
"use client";
import type React from "react";
import type { ScriptHTMLAttributes } from "react";
import { useEffect, useRef } from "react";
import ReactDOM from "react-dom";
import { requestIdleCallback, setAttributesFromProps } from "./utils";
const ScriptCache = new Map<string, Promise<Event>>();
const LoadCache = new Set<string>();
const insertedStylesheets = new Set<string>();
export interface ScriptProps extends ScriptHTMLAttributes<HTMLScriptElement> {
    strategy?: "afterInteractive" | "lazyOnload" | "beforeInteractive";
    id?: string;
    onLoad?: (e: any) => void;
    onReady?: () => void | null;
    onError?: (e: any) => void;
    children?: React.ReactNode;
    stylesheets?: string[];
}
const safePreinit = (href: string, options: {
    as: "style" | "script";
    [key: string]: any;
}, precedence: "reset" | "low" | "medium" | "high" = "medium")=>{
    const preinitFn = (ReactDOM as any).preinit || (ReactDOM as any).experimental_preinit;
    if (typeof preinitFn === "function") {
        preinitFn(href, {
            precedence: precedence,
            ...options
        });
    }
};
const safePreload = (href: string, options: {
    as: "style" | "script";
    [key: string]: any;
})=>{
    const preloadFn = (ReactDOM as any).preload || (ReactDOM as any).experimental_preload;
    if (typeof preloadFn === "function") {
        preloadFn(href, options);
    }
};
const insertStylesheets = (stylesheets: string[])=>{
    const preinitFn = (ReactDOM as any).preinit || (ReactDOM as any).experimental_preinit;
    if (typeof preinitFn === "function") {
        stylesheets.forEach((stylesheet: string)=>{
            safePreinit(stylesheet, {
                as: "style"
            });
        });
        return;
    }
    if (typeof window !== "undefined") {
        const head = document.head;
        stylesheets.forEach((stylesheet: string)=>{
            if (insertedStylesheets.has(stylesheet)) return;
            if (head.querySelector(`link[href="${stylesheet}"]`)) {
                insertedStylesheets.add(stylesheet);
                return;
            }
            const link = document.createElement("link");
            link.type = "text/css";
            link.rel = "stylesheet";
            link.href = stylesheet;
            head.appendChild(link);
            insertedStylesheets.add(stylesheet);
        });
    }
};
const loadScript = (props: ScriptProps): void =>{
    const { src, id, onLoad, onReady = null, dangerouslySetInnerHTML, children = "", strategy = "afterInteractive", onError, stylesheets, ...restProps } = props;
    const cacheKey = id || src;
    if (cacheKey && LoadCache.has(cacheKey)) {
        return;
    }
    const afterLoad = ()=>{
        if (onReady) {
            onReady();
        }
        if (cacheKey) {
            LoadCache.add(cacheKey);
        }
    };
    if (src && ScriptCache.has(src)) {
        if (cacheKey) {
            LoadCache.add(cacheKey);
        }
        const cachedPromise = ScriptCache.get(src);
        if (cachedPromise) {
            cachedPromise.then((e)=>{
                if (onLoad) {
                    onLoad(e);
                }
                afterLoad();
            }, onError);
        }
        return;
    }
    const el = document.createElement("script");
    const loadPromise = new Promise<Event>((resolve, reject)=>{
        el.addEventListener("load", function(e) {
            resolve(e);
            if (onLoad) {
                onLoad.call(this, e);
            }
            afterLoad();
        });
        el.addEventListener("error", (e)=>{
            if (src) {
                ScriptCache.delete(src);
            }
            reject(e);
        });
    });
    loadPromise.catch((e)=>{
        if (onError) {
            onError(e);
        }
    });
    if (dangerouslySetInnerHTML) {
        el.innerHTML = (dangerouslySetInnerHTML.__html as string) || "";
        afterLoad();
    } else if (children) {
        el.textContent = typeof children === "string" ? children : Array.isArray(children) ? children.join("") : "";
        afterLoad();
    } else if (src) {
        el.src = src;
        ScriptCache.set(src, loadPromise);
    }
    setAttributesFromProps(el, restProps);
    el.setAttribute("data-rspress-script", strategy);
    if (stylesheets) {
        insertStylesheets(stylesheets);
    }
    document.body.appendChild(el);
};
export function handleClientScriptLoad(props: ScriptProps) {
    const { strategy = "afterInteractive" } = props;
    if (strategy === "lazyOnload") {
        window.addEventListener("load", ()=>{
            requestIdleCallback(()=>loadScript(props));
        });
    } else {
        loadScript(props);
    }
}
function loadLazyScript(props: ScriptProps) {
    if (document.readyState === "complete") {
        requestIdleCallback(()=>loadScript(props));
    } else {
        window.addEventListener("load", ()=>{
            requestIdleCallback(()=>loadScript(props));
        });
    }
}
function addBeforeInteractiveToCache() {
    if (typeof document === "undefined") return;
    const scripts = document.querySelectorAll('[data-rspress-script="beforeInteractive"]');
    scripts.forEach((script)=>{
        const cacheKey = script.id || script.getAttribute("src");
        if (cacheKey) {
            LoadCache.add(cacheKey);
        }
    });
}
export function initScriptLoader(scriptLoaderItems: ScriptProps[]) {
    addBeforeInteractiveToCache();
    scriptLoaderItems.forEach(handleClientScriptLoad);
}
export function Script(props: ScriptProps): React.JSX.Element | null {
    const { id, src = "", onLoad, onReady = null, strategy = "afterInteractive", onError, stylesheets, nonce, dangerouslySetInnerHTML, children, ...restProps } = props;
    const cacheKey = id || src;
    const lastLoadedSrcOrId = useRef<string | null>(null);
    useEffect(()=>{
        if (lastLoadedSrcOrId.current !== cacheKey) {
            if (onReady && cacheKey && LoadCache.has(cacheKey)) {
                onReady();
            }
            lastLoadedSrcOrId.current = cacheKey;
        }
    }, [
        onReady,
        cacheKey
    ]);
    const lastInitializedKey = useRef<string | null>(null);
    const inlineContent = dangerouslySetInnerHTML?.__html || (typeof children === "string" ? children : "");
    useEffect(()=>{
        if (lastInitializedKey.current !== cacheKey) {
            addBeforeInteractiveToCache();
            if (strategy === "afterInteractive") {
                loadScript(props);
            } else if (strategy === "lazyOnload") {
                loadLazyScript(props);
            } else if (strategy === "beforeInteractive") {
                loadScript(props);
            }
            lastInitializedKey.current = cacheKey;
        }
    }, [
        cacheKey,
        strategy,
        inlineContent
    ]);
    if (stylesheets) {
        stylesheets.forEach((styleSrc)=>{
            safePreinit(styleSrc, {
                as: "style",
                precedence: "medium"
            });
        });
    }
    if (typeof window === "undefined") {
        if (src && (strategy === "beforeInteractive" || strategy === "afterInteractive")) {
            safePreload(src, restProps.integrity ? {
                as: "script",
                integrity: restProps.integrity,
                nonce,
                crossOrigin: restProps.crossOrigin
            } : {
                as: "script",
                nonce,
                crossOrigin: restProps.crossOrigin
            });
        }
        if (strategy === "beforeInteractive") {
            if (!src) {
                const innerHTML = dangerouslySetInnerHTML ? (dangerouslySetInnerHTML.__html as string) : typeof children === "string" ? children : Array.isArray(children) ? children.join("") : "";
                return (<script nonce={nonce} dangerouslySetInnerHTML={{
                    __html: innerHTML
                }} data-rspress-script="beforeInteractive" {...restProps}/>);
            } else {
                return (<script src={src} nonce={nonce} data-rspress-script="beforeInteractive" {...restProps}/>);
            }
        }
        return null;
    }
    return null;
}

```

### File: `src/components/utils.ts`

**Function Summaries**:
- Line 39: `isBooleanScriptAttribute` -> *Checks if a given attribute string matches one of the recognized boolean attributes for scripting elements.*
- Line 45: `setAttributesFromProps` -> *Synchronizes the HTML element's attributes using key-value pairs from a props object, applying type conversion while providing specialized logic to set and subsequently remove attributes for boolean or explicitly false values.*

```typescript
export const requestIdleCallback = (typeof self !== "undefined" && self.requestIdleCallback && self.requestIdleCallback.bind(window)) || ((cb: IdleRequestCallback): number =>{
    const start = Date.now();
    return self.setTimeout(()=>{
        cb({
            didTimeout: false,
            timeRemaining: ()=>Math.max(0, 50 - (Date.now() - start))
        });
    }, 1);
});
export const cancelIdleCallback = (typeof self !== "undefined" && self.cancelIdleCallback && self.cancelIdleCallback.bind(window)) || ((id: number)=>clearTimeout(id));
const DOMAttributeNames: Record<string, string> = {
    acceptCharset: "accept-charset",
    className: "class",
    htmlFor: "for",
    httpEquiv: "http-equiv",
    noModule: "noModule"
};
const ignoreProps = [
    "onLoad",
    "onReady",
    "dangerouslySetInnerHTML",
    "children",
    "onError",
    "strategy",
    "stylesheets"
];
function isBooleanScriptAttribute(attr: string): attr is "async" | "defer" | "noModule" {
    return [
        "async",
        "defer",
        "noModule"
    ].includes(attr);
}
export function setAttributesFromProps(el: HTMLElement, props: object) {
    for (const [p, value] of Object.entries(props)){
        if (!Object.hasOwn(props, p)) continue;
        if (ignoreProps.includes(p)) continue;
        if (value === undefined) continue;
        const attr = DOMAttributeNames[p] || p.toLowerCase();
        if (el.tagName === "SCRIPT" && isBooleanScriptAttribute(attr)) {
            (el as HTMLScriptElement)[attr] = !!value;
        } else {
            el.setAttribute(attr, String(value));
        }
        if (value === false || (el.tagName === "SCRIPT" && isBooleanScriptAttribute(attr) && (!value || value === "false"))) {
            el.setAttribute(attr, "");
            el.removeAttribute(attr);
        }
    }
}

```

### File: `src/components/ThirdPartyScripts.tsx`

```typescript
"use client";
export type ScriptEmbed = {
    html?: string | null;
    height?: string | number | null;
    width?: string | number | null;
    children?: React.ReactElement | React.ReactElement[];
};
export default function ThirdPartyScriptEmbed({ html, height = null, width = null, children }: ScriptEmbed) {
    return (<>
			{}
			{children}
			{}
			{html ? (<div style={{
        height: height != null ? `${height}px` : "auto",
        width: width != null ? `${width}px` : "auto"
    }} dangerouslySetInnerHTML={{
        __html: html
    }}/>) : null}
		</>);
}

```

### File: `src/components/GoogleMapsEmbed.tsx`

**Function Summaries**:
- Line 21: `GoogleMapsEmbed` -> *Renders a third-party Google Maps embed component by substituting the API key and passing all other provided configuration props.*

```typescript
import { GoogleMapsEmbed as TPCGoogleMapEmbed } from "third-party-capital";
import ThirdPartyScriptEmbed from "./ThirdPartyScripts";
export type GoogleMapsEmbedTypes = {
    height?: number | string;
    width?: number | string;
    mode: "place" | "view" | "directions" | "streetview" | "search";
    apiKey: string;
    style?: string;
    allowfullscreen?: boolean;
    loading?: "eager" | "lazy";
    q?: string;
    id?: string;
    center?: string;
    zoom?: string;
    maptype?: string;
    language?: string;
    region?: string;
};
export function GoogleMapsEmbed(props: GoogleMapsEmbedTypes) {
    const { apiKey, ...restProps } = props;
    const formattedProps = {
        ...restProps,
        key: apiKey
    };
    const { html } = TPCGoogleMapEmbed(formattedProps);
    return (<ThirdPartyScriptEmbed height={formattedProps.height || null} width={formattedProps.width || null} html={html}/>);
}

```

### File: `src/components/GoogleTagManager.tsx`

**Function Summaries**:
- Line 32: `GoogleTagManager` -> *Initializes and loads the specified Google Tag Manager script, optionally including custom data layers and preview parameters for tracking.*

```typescript
"use client";
import { Script } from "./Script";
type JSONValue = string | number | boolean | JSONValue[] | {
    [key: string]: JSONValue;
};
type GTMParamsBaseParams = {
    dataLayer?: {
        [key: string]: JSONValue;
    };
    dataLayerName?: string;
    auth?: string;
    preview?: string;
    nonce?: string;
};
type GTMParamsWithId = GTMParamsBaseParams & {
    gtmId: string;
    gtmScriptUrl?: string;
};
type GTMParamsWithScriptUrl = GTMParamsBaseParams & {
    gtmId?: string;
    gtmScriptUrl: string;
};
export type GTMParams = GTMParamsWithId | GTMParamsWithScriptUrl;
export function GoogleTagManager(props: GTMParams) {
    const { gtmId, gtmScriptUrl, dataLayerName = "dataLayer", auth, preview, dataLayer, nonce } = props;
    const scriptUrl = new URL(gtmScriptUrl || "https://www.googletagmanager.com/gtm.js");
    if (gtmId) {
        scriptUrl.searchParams.set("id", gtmId);
    }
    if (dataLayerName !== "dataLayer") {
        scriptUrl.searchParams.set("l", dataLayerName);
    }
    if (auth) {
        scriptUrl.searchParams.set("gtm_auth", auth);
    }
    if (preview) {
        scriptUrl.searchParams.set("gtm_preview", preview);
        scriptUrl.searchParams.set("gtm_cookies_win", "x");
    }
    return (<>
			<Script id="_rspress-gtm-init" dangerouslySetInnerHTML={{
        __html: `
      (function(w,l){
        w[l]=w[l]||[];
        w[l].push({'gtm.start': new Date().getTime(),event:'gtm.js'});
        ${dataLayer ? `w[l].push(${JSON.stringify(dataLayer)})` : ""}
      })(window,'${dataLayerName}');`
    }} nonce={nonce}/>
			<Script id="_rspress-gtm" src={scriptUrl.href} nonce={nonce}/>
		</>);
}

```

### File: `src/components/TweetEmbed.tsx`

```typescript
import { useDark } from "@rspress/core/runtime";
import type { ReactNode } from "react";
import { Tweet, type TweetProps } from "react-tweet";
export type TweetEmbedTypes = TweetProps & {
    theme?: "light" | "dark";
    caption?: ReactNode;
};
export const TweetEmbed = (props: TweetEmbedTypes)=>{
    const isDark = useDark();
    const defaultTheme = isDark ? "dark" : "light";
    const { theme = defaultTheme, caption, ...restProps } = props;
    return (<figure data-theme={theme}>
			<Tweet {...restProps}/>
			{caption && <figcaption>{caption}</figcaption>}
		</figure>);
};

```

### File: `src/plugin.ts`

**Function Summaries**:
- Line 21: `pluginThirdParties` -> *Initializes and returns the plugin configuration object, conditionally including Google Analytics components and defining its global configuration variables based on provided options.*

```typescript
import path from "node:path";
import { fileURLToPath } from "node:url";
import type { RspressPlugin } from "@rspress/core";
export interface ThirdPartiesPluginOptions {
    googleAnalytics?: {
        gaId: string;
        dataLayerName?: string;
        debugMode?: boolean;
        nonce?: string;
    };
}
const getDirname = ()=>{
    if (typeof __dirname !== "undefined") {
        return __dirname;
    }
    return path.dirname(fileURLToPath(import.meta.url));
};
export function pluginThirdParties(options: ThirdPartiesPluginOptions = {}): RspressPlugin {
    const globalUIComponents = [];
    if (options.googleAnalytics) {
        globalUIComponents.push(path.join(getDirname(), "GlobalGoogleAnalytics.js"));
    }
    const gaConfigDefine = options.googleAnalytics ? JSON.stringify(JSON.stringify(options.googleAnalytics)) : "null";
    return {
        name: "rspress-plugin-third-parties",
        globalUIComponents,
        builderConfig: {
            source: {
                define: {
                    "process.env.RSPRESS_GA_CONFIG": gaConfigDefine
                }
            }
        }
    };
}

```

### File: `examples/demo/rspress.config.ts`

```typescript
import path from "node:path";
import { defineConfig } from "@rspress/core";
import { pluginThirdParties } from "rspress-plugin-third-parties/plugin";
export default defineConfig({
    root: path.join(__dirname, "docs"),
    title: "Rspress Third Parties Demo",
    siteOrigin: "https://sanjaiyan-dev.github.io",
    base: "/rspress-plugin-third-parties/",
    llms: true,
    description: "Live interactive playground for rspress-plugin-third-parties",
    icon: "/rspress-plugin-third-party-icon.png",
    logo: "/rspress-plugin-third-party-icon.png",
    head: [
        [
            "meta",
            {
                property: "og:image",
                content: "https://sanjaiyan-dev.github.io/rspress-plugin-third-parties/rspress-plugin-third-party-hero.png"
            }
        ]
    ],
    logoText: "Rspress Plugin Third Parties",
    themeConfig: {
        socialLinks: [
            {
                icon: "github",
                mode: "link",
                content: "https://github.com/sanjaiyan-dev/rspress-plugin-third-parties"
            },
            {
                icon: "npm",
                mode: "link",
                content: "https://www.npmjs.com/package/rspress-plugin-third-parties"
            },
            {
                icon: "instagram",
                mode: "link",
                content: "https://www.instagram.com/sanjaiyan_dev"
            }
        ],
        enableAppearanceAnimation: true,
        enableContentAnimation: true
    },
    plugins: [
        pluginThirdParties({
            googleAnalytics: {
                gaId: "GA-ID"
            }
        })
    ],
    markdown: {
        link: {
            checkDeadLinks: false
        }
    }
});

```


