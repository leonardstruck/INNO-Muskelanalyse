import Sidebar from './Sidebar'


type MainLayoutProps = {
    children: React.ReactNode
}

const MainLayout = ({ children }: MainLayoutProps) => {

    return (
        <>
            <div className="h-full">
                <Sidebar />
                <main className="py-10 lg:pl-72 h-full">
                    <div className="px-4 sm:px-6 lg:px-8 h-full">{children}</div>
                </main>
            </div>
        </>
    )
}


export default MainLayout;