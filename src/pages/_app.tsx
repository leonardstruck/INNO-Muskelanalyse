import type { AppProps } from "next/app";

import "../style.css";

import { Teko, Rubik } from "next/font/google";
import clsx from "clsx";
import { NextPage } from "next";
import MainLayout from "../components/layouts/main";
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

const rubik = Rubik({
  variable: "--font-rubik",
  subsets: ["latin"],
  weight: "variable"
})


export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
  getLayout?: (page: React.ReactElement) => React.ReactNode
}

type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout
}


export default function MyApp({ Component, pageProps }: AppPropsWithLayout) {
  // use the layout defined at the page level, if available
  const getLayout = Component.getLayout ?? ((page) => <MainLayout>{page}</MainLayout>)
  const queryClient = new QueryClient()

  return (
    <QueryClientProvider client={queryClient}>
      <div className={clsx(rubik.variable, "h-full font-sans")}>
        {getLayout(<Component {...pageProps} />)}
      </div>
    </QueryClientProvider>
  );
}
