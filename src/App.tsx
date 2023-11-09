import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Inbox, Plus, Trash } from 'lucide-react';

import { Input } from '@/components/ui/input';
import { Button } from './components/ui/button';
import { cn, formatDate } from './lib/utils';
import { ScrollArea } from './components/ui/scroll-area';

type Todo = {
    id: string;
    text: string;
    is_completed: boolean;
    created_at: string;
};

function InputBox() {
    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();
        const formData = new FormData(e.target as HTMLFormElement);
        const todo = formData.get('todo');
        await invoke('create_todo', {
            todo: {
                text: todo,
            },
        });
        location.reload();
    }

    return (
        <form
            className="flex w-full items-center gap-2"
            onSubmit={handleSubmit}
        >
            <Input
                placeholder="Type your todo"
                name="todo"
                autoComplete="off"
            />
            <Button size="icon" className="w-12">
                <Plus />
            </Button>
        </form>
    );
}

function Todos({ todos }: { todos: Todo[] }) {
    return (
        <ScrollArea className="mt-8 h-full w-full">
            <div className="flex flex-col gap-2">
                {todos.map((todo) => (
                    <div
                        key={todo.id}
                        className="flex items-center justify-between border px-4 py-2"
                    >
                        <Button
                            variant="ghost"
                            size="lg"
                            onClick={async () => {
                                await invoke('toggle_todo_status', {
                                    id: todo.id,
                                });
                                location.reload();
                            }}
                            className="flex w-full flex-col items-start p-0 hover:bg-transparent"
                        >
                            <p
                                className={cn({
                                    'line-through': todo.is_completed,
                                })}
                            >
                                {todo.text}
                            </p>
                            <time className="text-sm text-slate-400">
                                {formatDate(todo.created_at)}
                            </time>
                        </Button>
                        <Button
                            variant="link"
                            className="p-0"
                            onClick={async () => {
                                await invoke('delete_todo', { id: todo.id });
                                location.reload();
                            }}
                        >
                            <Trash className="text-red-500" />
                        </Button>
                    </div>
                ))}
            </div>
        </ScrollArea>
    );
}

function App() {
    const [todos, setTodos] = useState<Todo[]>([]);

    useEffect(() => {
        (async () => {
            const todos = (await invoke('get_todos')) as Todo[];
            console.log('todos: ', todos);
            if (todos) setTodos(todos);
        })();
    }, []);

    return (
        <div className="mx-auto grid min-h-screen w-full max-w-xl grid-rows-[auto_auto_1fr] place-items-center gap-4 px-4 py-8">
            <h1 className="justify-self-start text-2xl font-bold">
                Your todos
            </h1>
            <InputBox />
            {/* todos */}
            {todos.length > 0 ? (
                <Todos todos={todos} />
            ) : (
                <div className="flex h-40 w-full max-w-xs flex-col items-center justify-center rounded-md bg-slate-100">
                    <Inbox size={38} />
                    <p className="text-xl">List is empty</p>
                </div>
            )}
        </div>
    );
}

export default App;
