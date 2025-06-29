'use client';

import { useTransition } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useAuthModal } from '@/stores/use-auth-modal';
import { loginAction } from '@/app/actions/auth.actions';

import { toast } from 'sonner';

import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';

const loginSchema = z.object({
  userName: z.string().min(1, "Username is required"),
  userPassword: z.string().min(1, "Password is required"),
});

export function LoginForm() {
  const [isPending, startTransition] = useTransition();
  const { setView, closeModal } = useAuthModal();

  const form = useForm<z.infer<typeof loginSchema>>({
    resolver: zodResolver(loginSchema),
    defaultValues: { userName: '', userPassword: '' },
  });

  const onSubmit = (values: z.infer<typeof loginSchema>) => {
    startTransition(async () => {
      const result = await loginAction(values);
      if (result.success) {
        toast.success(result.message);

        closeModal();
        window.location.reload(); // Simple way to refresh logged-in state
      } else {
        toast.error(result.message);
      }
    });
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
        <FormField
          control={form.control}
          name="userName"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Username</FormLabel>
              <FormControl>
                <Input placeholder="your_username" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="userPassword"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Password</FormLabel>
              <FormControl>
                <Input type="password" placeholder="••••••••" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" className="w-full" disabled={isPending}>
          {isPending ? 'Logging in...' : 'Log In'}
        </Button>
      </form>
      <div className="mt-4 text-center text-sm">
        Don't have an account?{' '}
        <Button variant="link" className="p-0 h-auto" onClick={() => setView('register')}>
          Sign up
        </Button>
      </div>
    </Form>
  );
}
