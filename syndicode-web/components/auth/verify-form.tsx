'use client';

import { useEffect } from 'react';
import { useTransition } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { toast } from 'sonner';

import { useAuthModal } from '@/stores/use-auth-modal';
import { verifyUserAction, resendCodeAction } from '@/app/actions/auth.actions';

import { Button } from '@/components/ui/button';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { InputOTP, InputOTPGroup, InputOTPSlot } from '@/components/ui/input-otp';

const verifySchema = z.object({
  code: z.string().length(10, "Verification code must be 10 characters."),
});

export function VerifyForm() {
  const [isVerifying, startVerifyTransition] = useTransition();
  const [isResending, startResendTransition] = useTransition();
  const { setView, userNameToVerify } = useAuthModal();

  const form = useForm<z.infer<typeof verifySchema>>({
    resolver: zodResolver(verifySchema),
    defaultValues: { code: '' },
  });

  // Graceful exit: If the user reloads or lands here without a username,
  // send them back to the login page.
  useEffect(() => {
    if (!userNameToVerify) {
      setView('login');
    }
  }, [userNameToVerify, setView]);

  const onSubmit = (values: z.infer<typeof verifySchema>) => {
    if (!userNameToVerify) {
      toast.error("Something went wrong. Please try registering again.");
      setView('register');
      return;
    }

    startVerifyTransition(async () => {
      const result = await verifyUserAction({
        userName: userNameToVerify,
        code: values.code,
      });

      if (result.success) {
        toast.success(result.message);
        setView('login'); // Success! Send them to the login form.
      } else {
        toast.error(result.message);
      }
    });
  };

  const handleResendCode = () => {
    if (!userNameToVerify) return;

    startResendTransition(async () => {
      const result = await resendCodeAction(userNameToVerify);
      if (result.success) {
        toast.success(result.message);
      } else {
        toast.error(result.message);
      }
    });
  };

  if (!userNameToVerify) {
    // Render nothing while the useEffect redirects, or show a loading state.
    return null;
  }

  return (
    <div className="flex flex-col items-center justify-center">
      <p className="text-sm text-muted-foreground mb-4">
        Enter the 10-digit code sent to your email for <span className="font-semibold">{userNameToVerify}</span>.
      </p>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <FormField
            control={form.control}
            name="code"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Verification Code</FormLabel>
                <FormControl>
                  <InputOTP maxLength={10} autoFocus {...field}>
                    <InputOTPGroup>
                      <InputOTPSlot index={0} />
                      <InputOTPSlot index={1} />
                      <InputOTPSlot index={2} />
                      <InputOTPSlot index={3} />
                      <InputOTPSlot index={4} />
                      <InputOTPSlot index={5} />
                      <InputOTPSlot index={6} />
                      <InputOTPSlot index={7} />
                      <InputOTPSlot index={8} />
                      <InputOTPSlot index={9} />
                    </InputOTPGroup>
                  </InputOTP>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <Button type="submit" className="w-full" disabled={isVerifying || isResending}>
            {isVerifying ? 'Verifying...' : 'Verify Account'}
          </Button>
        </form>
      </Form>
      <div className="mt-4 text-center text-sm">
        Didn&apos;t receive a code?{' '}
        <Button
          variant="link"
          className="p-0 h-auto"
          onClick={handleResendCode}
          disabled={isVerifying || isResending}
        >
          {isResending ? 'Sending...' : 'Resend'}
        </Button>
      </div>
    </div>
  );
}
