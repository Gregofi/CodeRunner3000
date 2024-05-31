import { Header } from './header';

export function Layout({ children }: Readonly<{ children: React.ReactNode }>) {
    return (
        <div className="h-full flex flex-col">
            <Header />
            {children}
        </div>
    );
}
