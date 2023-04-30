import type { AppProps } from "next/app";

import "../style.css";

import { Teko, Rubik } from "next/font/google";
import clsx from "clsx";

const teko = Teko({
  variable: "--font-teko",
  subsets: ["latin"],
  weight: "400"
})

const rubik = Rubik({
  variable: "--font-rubik",
  subsets: ["latin"],
  weight: "variable"
})

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  return <div className={clsx(teko.variable, rubik.variable, "h-full font-sans")}><Component {...pageProps} /></div>;
}
