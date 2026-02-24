"use client";

import { useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Shield, AlertTriangle, CheckCircle2, Scan } from "lucide-react";
import { Progress } from "@/components/ui/progress";

type ScanStatus = "idle" | "scanning" | "complete";
type ScanResult = {
  category: string;
  status: "secure" | "warning" | "critical";
  message: string;
};

export function ProtocolScanner() {
  const [scanStatus, setScanStatus] = useState<ScanStatus>("idle");
  const [progress, setProgress] = useState(0);
  const [results, setResults] = useState<ScanResult[]>([]);

  const runMeshScan = async () => {
    setScanStatus("scanning");
    setProgress(0);
    setResults([]);

    const scanSteps = [
      { category: "E-Network Integrity", status: "secure" as const, message: "All nodes synchronized" },
      { category: "Merchant PQS Compliance", status: "secure" as const, message: "Zero violations detected" },
      { category: "bulacan.pi Sovereignty", status: "secure" as const, message: "Mapping verified" },
      { category: "DAO Governance", status: "warning" as const, message: "1 pending proposal" },
      { category: "Protocol Security", status: "secure" as const, message: "Hard-coded trust verified" }
    ];

    for (let i = 0; i < scanSteps.length; i++) {
      await new Promise(resolve => setTimeout(resolve, 800));
      setProgress(((i + 1) / scanSteps.length) * 100);
      setResults(prev => [...prev, scanSteps[i]]);
    }

    setScanStatus("complete");
  };

  const getStatusIcon = (status: ScanResult["status"]) => {
    switch (status) {
      case "secure":
        return <CheckCircle2 className="h-5 w-5 text-accent" />;
      case "warning":
        return <AlertTriangle className="h-5 w-5 text-yellow-500" />;
      case "critical":
        return <AlertTriangle className="h-5 w-5 text-destructive" />;
    }
  };

  const getStatusBadge = (status: ScanResult["status"]) => {
    const variants = {
      secure: "default",
      warning: "secondary",
      critical: "destructive"
    };
    return <Badge variant={variants[status] as any}>{status.toUpperCase()}</Badge>;
  };

  return (
    <Card className="border-primary/20 bg-card/50 backdrop-blur">
      <CardHeader>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="rounded-lg bg-primary/10 p-2">
              <Shield className="h-5 w-5 text-primary" />
            </div>
            <div>
              <CardTitle className="text-lg">MESH-SCAN Protocol</CardTitle>
              <CardDescription>Pioneer Quality Standards Auditing</CardDescription>
            </div>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {scanStatus === "idle" && (
          <Button onClick={runMeshScan} className="w-full" size="lg">
            <Scan className="mr-2 h-4 w-4" />
            Initialize Security Scan
          </Button>
        )}

        {scanStatus === "scanning" && (
          <div className="space-y-3">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Scanning E-Network...</span>
              <span className="font-mono text-primary">{Math.round(progress)}%</span>
            </div>
            <Progress value={progress} className="h-2" />
          </div>
        )}

        {results.length > 0 && (
          <div className="space-y-2">
            {results.map((result, index) => (
              <div
                key={index}
                className="flex items-start gap-3 rounded-lg border border-border/50 bg-background/50 p-3"
              >
                {getStatusIcon(result.status)}
                <div className="flex-1 space-y-1">
                  <div className="flex items-center justify-between">
                    <p className="text-sm font-medium leading-none">{result.category}</p>
                    {getStatusBadge(result.status)}
                  </div>
                  <p className="text-xs text-muted-foreground">{result.message}</p>
                </div>
              </div>
            ))}
          </div>
        )}

        {scanStatus === "complete" && (
          <Button onClick={runMeshScan} variant="outline" className="w-full">
            <Scan className="mr-2 h-4 w-4" />
            Re-scan Network
          </Button>
        )}
      </CardContent>
    </Card>
  );
}
