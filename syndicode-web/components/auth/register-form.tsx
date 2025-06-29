'use client';

import { useTransition } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { toast } from 'sonner';

import { useAuthModal } from '@/stores/use-auth-modal';
import { registerAction } from '@/app/actions/auth.actions';

import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';

const registerSchema = z.object({
  userName: z.string().min(3, "Username must be at least 3 characters."),
  email: z.string().email("Please enter a valid email address."),
  corporationName: z.string().min(1, "Corporation name is required."),
  userPassword: z.string().min(6, "Password must be at least 6 characters."),
});

export function RegisterForm() {
  const [isPending, startTransition] = useTransition();
  const { setView, setUserNameToVerify } = useAuthModal();

  const form = useForm<z.infer<typeof registerSchema>>({
    resolver: zodResolver(registerSchema),
    defaultValues: {
      userName: '',
      email: '',
      corporationName: '',
      userPassword: '',
    },
  });

  const onSubmit = (values: z.infer<typeof registerSchema>) => {
    startTransition(async () => {
      const result = await registerAction(values);

      if (result.success) {
        toast.success(result.message);
        // Pass the username to the store so the verify form knows who to verify
        setUserNameToVerify(values.userName);
        // Switch the modal to the 'verify' view
        setView('verify');
      } else {
        toast.error(result.message);
      }
    });
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
        {/* Username */}
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
        {/* Email */}
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Email</FormLabel>
              <FormControl>
                <Input type="email" placeholder="you@company.com" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        {/* Corporation Name */}
        <FormField
          control={form.control}
          name="corporationName"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Corporation Name</FormLabel>
              <FormControl>
                <Input placeholder="Your Corp" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        {/* Password */}
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
          {isPending ? 'Registering...' : 'Create Account'}
        </Button>
      </form>
      <div className="mt-4 text-center text-sm">
        Already have an account?{' '}
        <Button variant="link" className="p-0 h-auto" onClick={() => setView('login')}>
          Log in
        </Button>
      </div>
    </Form>
  );
}
