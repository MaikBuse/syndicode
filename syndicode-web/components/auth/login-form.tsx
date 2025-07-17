'use client';

import { useState, useTransition } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useAuthModal } from '@/stores/use-auth-modal';
import { loginAction } from '@/app/actions/auth.actions';
import { Eye, EyeOff } from 'lucide-react';

import { toast } from 'sonner';

import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { useAuthStore } from '@/stores/use-auth-store';

const loginSchema = z.object({
  userName: z.string().min(1, "Username is required"),
  userPassword: z.string().min(1, "Password is required"),
});

export function LoginForm() {
  const [isPending, startTransition] = useTransition();
  const [showPassword, setShowPassword] = useState(false);
  const { setView, closeModal, setUserNameToVerify } = useAuthModal();

  const form = useForm<z.infer<typeof loginSchema>>({
    resolver: zodResolver(loginSchema),
    defaultValues: { userName: '', userPassword: '' },
  });

  const onSubmit = (values: z.infer<typeof loginSchema>) => {
    startTransition(async () => {
      const loginResult = await loginAction(values);
      
      if (loginResult.isInactive) {
        // User needs to verify their account
        setUserNameToVerify(values.userName);
        setView('verify');
        toast.info(loginResult.message);
        return;
      }
      
      if (loginResult.success) {
        toast.success(loginResult.message);
        if (loginResult.user) {
          useAuthStore.getState().login(loginResult.user);
        }
        closeModal();
      } else {
        toast.error(loginResult.message);
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
                <Input placeholder="your_username" autoFocus {...field} />
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
                <div className="relative">
                  <Input
                    type={showPassword ? "text" : "password"}
                    placeholder="••••••••"
                    {...field}
                  />
                  <button
                    type="button"
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      setShowPassword(!showPassword);
                    }}
                    className="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-500 hover:text-gray-700 focus:outline-none"
                  >
                    {showPassword ? (
                      <EyeOff className="h-4 w-4" />
                    ) : (
                      <Eye className="h-4 w-4" />
                    )}
                  </button>
                </div>
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
        Don&apos;t have an account?{' '}
        <Button variant="link" className="p-0 h-auto" onClick={() => setView('register')}>
          Sign up
        </Button>
      </div>
    </Form>
  );
}
