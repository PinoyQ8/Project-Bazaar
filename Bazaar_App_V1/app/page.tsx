"use client";

import { usePiAuth } from "@/contexts/pi-auth-context";
import { ProtocolScanner } from "@/components/protocol-scanner";
import { GovernancePanel } from "@/components/governance-panel";
import { NetworkStatus } from "@/components/network-status";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Hexagon, MapPin, Package, User } from "lucide-react";

export default function HomePage() {
  const { userData } = usePiAuth();

  return (
    <div className="min-h-screen bg-background">
      <main className="mx-auto max-w-md space-y-4 p-4 pb-8">
        {/* Header */}
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="relative">
                <Hexagon className="h-10 w-10 text-primary" fill="currentColor" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <Hexagon className="h-6 w-6 text-background" fill="currentColor" />
                </div>
              </div>
              <div>
                <h1 className="text-xl font-bold leading-none text-balance">
                  Bazaar Pioneers
                </h1>
                <p className="text-sm text-muted-foreground">DAO Gateway</p>
              </div>
            </div>
            <Badge variant="outline" className="border-primary/30 bg-primary/5 font-mono">
              v2.0
            </Badge>
          </div>

          <div className="rounded-lg border border-primary/30 bg-primary/5 p-3">
            <p className="text-xs leading-relaxed text-foreground/90 text-balance">
              Decentralized gateway for Project Bazaar. Powered by the MESH Protocol for secure
              E-Network access and PQS auditing.
            </p>
          </div>
        </div>

        {/* User Card */}
        {userData && (
          <Card className="border-primary/20 bg-card/50 backdrop-blur">
            <CardHeader className="pb-3">
              <div className="flex items-center gap-3">
                <div className="rounded-lg bg-primary/10 p-2">
                  <User className="h-5 w-5 text-primary" />
                </div>
                <div>
                  <CardTitle className="text-base">Pioneer Status</CardTitle>
                  <CardDescription className="text-xs">@{userData.username}</CardDescription>
                </div>
              </div>
            </CardHeader>
            <CardContent className="space-y-2">
              <div className="flex items-center justify-between rounded-lg border border-border/50 bg-background/50 p-3">
                <span className="text-sm text-muted-foreground">Credits Balance</span>
                <span className="font-mono text-lg font-bold text-primary">
                  {userData.credits_balance} π
                </span>
              </div>
              <div className="flex items-center justify-between rounded-lg border border-border/50 bg-background/50 p-3">
                <span className="text-sm text-muted-foreground">Status</span>
                <Badge variant="default" className="bg-accent text-accent-foreground">
                  VERIFIED
                </Badge>
              </div>
            </CardContent>
          </Card>
        )}

        {/* Network Status */}
        <NetworkStatus />

        {/* Protocol Scanner */}
        <ProtocolScanner />

        {/* Governance Panel */}
        <GovernancePanel />

        {/* Quick Actions */}
        <Card className="border-primary/20 bg-card/50 backdrop-blur">
          <CardHeader className="pb-3">
            <CardTitle className="text-lg">Pioneer Actions</CardTitle>
            <CardDescription>Access core protocol functions</CardDescription>
          </CardHeader>
          <CardContent className="space-y-2">
            <Button variant="outline" className="w-full justify-start" size="lg">
              <MapPin className="mr-3 h-4 w-4" />
              <div className="flex flex-col items-start">
                <span className="text-sm font-semibold">bulacan.pi Bridge</span>
                <span className="text-xs text-muted-foreground">Map physical utility</span>
              </div>
            </Button>
            <Button variant="outline" className="w-full justify-start" size="lg">
              <Package className="mr-3 h-4 w-4" />
              <div className="flex flex-col items-start">
                <span className="text-sm font-semibold">Merchant Registry</span>
                <span className="text-xs text-muted-foreground">View PQS-compliant traders</span>
              </div>
            </Button>
          </CardContent>
        </Card>

        {/* Footer */}
        <div className="rounded-lg border border-border/50 bg-muted/50 p-3 text-center">
          <p className="text-xs text-muted-foreground">
            <span className="font-semibold text-foreground">Built for Real Pioneers.</span>
            <br />
            Secure your bulacan.pi sovereignty. Hard-coded trust.
          </p>
        </div>
      </main>
    </div>
  );
}
