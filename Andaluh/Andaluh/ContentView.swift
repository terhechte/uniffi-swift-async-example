import SwiftUI

struct ContentView: View {
    @State private var output: String? = nil
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            if let output {
                Text(output)
            }
        }
        .padding()
        .onAppear {
            Task {
                let o = await expensive(input:"Estoy estudiante en Malaga")
                self.output = o
            }
        }
    }
}

#Preview {
    ContentView()
}
