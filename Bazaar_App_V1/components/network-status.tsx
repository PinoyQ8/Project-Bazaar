"use client";

import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Activity, Database, Lock, Zap } from "lucide-react";

export function NetworkStatus() {
  return (
    <Card className="border-primary/20 bg-card/50 backdrop-blur">
      <CardContent className="p-4">
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <div className="relative">
                <div className="absolute inset-0 animate-pulse rounded-full bg-accent/20" />
                <Activity className="relative h-4 w-4 text-accent" />
              </div>
              <span className="text-sm font-medium">E-Network Status</span>
            </div>
            <Badge variant="default" className="bg-accent text-accent-foreground">
              ONLINE
            </Badge>
          </div>

          <div className="grid grid-cols-2 gap-2">
            <div className="flex items-center gap-2 rounded-lg border border-border/50 bg-background/50 p-2">
              <Database className="h-4 w-4 text-primary" />
              <div className="flex flex-col">
                <span className="text-xs text-muted-foreground">Node Sync</span>
                <span className="text-sm font-semibold">100%</span>
              </div>
            </div>
            <div className="flex items-center gap-2 rounded-lg border border-border/50 bg-background/50 p-2">
              <Lock className="h-4 w-4 text-primary" />
              <div className="flex flex-col">
                <span className="text-xs text-muted-foreground">Security</span>
                <span className="text-sm font-semibold">Optimal</span>
              </div>
            </div>
          </div>

          <div className="flex items-center gap-2 rounded-lg border border-accent/30 bg-accent/5 p-2">
            <Zap className="h-4 w-4 text-accent" />
            <p className="text-xs text-foreground/90">
              <span className="font-semibold">MESH Protocol Active:</span> All systems operational
            </p>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
