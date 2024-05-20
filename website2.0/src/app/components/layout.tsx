import { Header } from './header';

function Layout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <>
            <Header />
            { children }
        </>
    );
}
