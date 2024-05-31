import { Layout } from '../components/layout';
import { Header } from '../components/header';
import dynamic from 'next/dynamic';

// SSR must be avoided for monaco-vim extension
const CodeRunner = dynamic(() => import('../components/codeRunner'), {
  ssr: false,
})

export default function Page() {
    return (
        <Layout>
            <CodeRunner />
        </Layout>
    );
}
